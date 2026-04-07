#![no_std]
#![no_main]

mod bargraph;
#[path = "bsp-ensea.rs"]
mod bsp_ensea;
mod encoder;
mod gamepad;
mod stepper;

use bargraph::Bargraph;
use bsp_ensea::Board;
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::Pull;
use embassy_stm32::{Config, bind_interrupts, exti};
use embassy_time::Timer;
use encoder::Encoder;
use stepper::{Direction, MicrostepMode, Stepper};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    EXTI15_10 => exti::InterruptHandler<embassy_stm32::interrupt::typelevel::EXTI15_10>;
});

#[embassy_executor::task]
async fn bargraph_task(mut bargraph: Bargraph<8>) {
    info!("bargraph_task démarrée");
    loop {
        bargraph.wait_and_update().await;
    }
}

#[embassy_executor::task]
async fn stepper_task(mut motor: Stepper) {
    info!("stepper_task démarrée");
    loop {
        motor.wait_and_update().await;
    }
}

#[embassy_executor::task]
async fn encoder_task(mut enc: Encoder) {
    info!("encoder_task démarrée");

    loop {
        Timer::after_millis(50).await;

        let pos = enc.position();

        let direction = if pos >= 0 {
            Direction::Clockwise
        } else {
            Direction::CounterClockwise
        };

        let abs_pos = pos.unsigned_abs();
        let speed_sps = abs_pos.saturating_mul(20).min(1000);
        let bar_level = abs_pos.saturating_mul(6) % 100;

        info!(
            "pos={} dir={} speed={} bar={}",
            pos,
            if pos >= 0 { "CW" } else { "CCW" },
            speed_sps,
            bar_level
        );

        Bargraph::<8>::update_value(bar_level);
        Stepper::update_speed(speed_sps, direction);
    }
}

#[embassy_executor::task]
async fn emergency_stop_task(mut btn: ExtiInput<'static>) {
    loop {
        btn.wait_for_falling_edge().await;
        info!("ARRET D'URGENCE");

        // Arrêt moteur
        Stepper::update_speed(0, Direction::Clockwise);

        Bargraph::<8>::update_value(0);

        // Désactivation et RAZ du timer encodeur (TIM2)
        let tim2 = embassy_stm32::pac::TIM2;
        tim2.cr1().modify(|w| w.set_cen(false));
        tim2.cnt().write_value(0);
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());
    let board = Board::new(p);

    // Bargraph
    let mut bargraph: Bargraph<8> = Bargraph::new([
        board.bargraph.led0,
        board.bargraph.led1,
        board.bargraph.led2,
        board.bargraph.led3,
        board.bargraph.led4,
        board.bargraph.led5,
        board.bargraph.led6,
        board.bargraph.led7,
    ]);
    bargraph.set_range(0, 100);

    let enc =
        Encoder::new_without_button(board.encoder.timer, board.encoder.ch_a, board.encoder.ch_b);

    let emergency_btn = ExtiInput::new(board.encoder.button, board.encoder.exti, Pull::Up, Irqs);

    let mut motor = Stepper::new(
        board.stepper.dir,
        board.stepper.ms1,
        board.stepper.ms2,
        board.stepper.enable,
        board.stepper.step,
        board.stepper.timer,
    );
    motor.set_microstep(MicrostepMode::Eighth);
    motor.enable();

    // Lancement des 4 tâches
    spawner.spawn(bargraph_task(bargraph)).unwrap();
    spawner.spawn(stepper_task(motor)).unwrap();
    spawner.spawn(encoder_task(enc)).unwrap();
    spawner.spawn(emergency_stop_task(emergency_btn)).unwrap();

    info!("Toutes les tâches démarrées");
}
