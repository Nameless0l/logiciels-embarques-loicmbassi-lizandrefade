#![no_std]
#![no_main]

#[path = "bsp-ensea.rs"]
mod bsp_ensea;
mod encoder;
mod stepper;

use bsp_ensea::Board;
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_time::Timer;
use encoder::Encoder;
use stepper::{Direction, MicrostepMode, Stepper};
use {defmt_rtt as _, panic_probe as _};

const MAX_SPEED: i32 = 500;
const MIN_SPS: u32 = 50;      // pas/s au niveau de vitesse minimum
const MAX_SPS: u32 = 1_000;   // pas/s au niveau de vitesse maximum

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
        board.stepper.timer,
    );

    let mut enc = Encoder::new(
        board.encoder.timer,
        board.encoder.ch_a,
        board.encoder.ch_b,
        board.encoder.button.into(),
    );

    motor.set_microstep(MicrostepMode::Eighth);
    motor.enable();

    let mut speed: i32 = 0;
    let mut prev_pos = enc.position();

    loop {
        // Bouton : arrêt immédiat et reset de la vitesse
        if enc.is_pressed() {
            speed = 0;
            motor.stop();
            enc.reset();
            prev_pos = 0;
            info!("Stop");
            while enc.is_pressed() {
                Timer::after_millis(10).await;
            }
        }

        // Delta encodeur → variation de vitesse
        let pos = enc.position();
        let delta = pos.wrapping_sub(prev_pos);
        prev_pos = pos;

        speed = speed.saturating_add(delta).clamp(-MAX_SPEED, MAX_SPEED);

        if speed == 0 {
            motor.stop();
        } else {
            let direction = if speed > 0 {
                Direction::Clockwise
            } else {
                Direction::CounterClockwise
            };

            // Interpolation linéaire MIN_SPS..MAX_SPS selon la valeur absolue de speed
            let abs_speed = speed.unsigned_abs().min(MAX_SPEED as u32);
            let sps = MIN_SPS.saturating_add(
                (MAX_SPS.saturating_sub(MIN_SPS)).saturating_mul(abs_speed) / MAX_SPEED as u32,
            );

            info!("speed={} sps={}", speed, sps);
            motor.set_speed(sps, direction);
        }

        Timer::after_millis(20).await;
    }
}
