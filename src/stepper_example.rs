#![no_std]
#![no_main]

#[path = "bsp-ensea.rs"]
mod bsp_ensea;
mod stepper;

use bsp_ensea::Board;
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_time::Timer;
use stepper::{Direction, MicrostepMode, Stepper};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());
    let board = Board::new(p);

    let mut motor = Stepper::new(
        board.stepper.dir,
        board.stepper.ms1,
        board.stepper.ms2,
        board.stepper.enable,
        board.stepper.step,
    );

    motor.set_microstep(MicrostepMode::Eighth);
    motor.enable();

    loop {
        info!("Sens horaire — 200 pas");
        motor.set_direction(Direction::Clockwise);
        motor.move_steps(200, 500).await;

        Timer::after_millis(500).await;

        info!("Sens anti-horaire — 200 pas");
        motor.set_direction(Direction::CounterClockwise);
        motor.move_steps(200, 500).await;

        Timer::after_millis(500).await;
    }
}
