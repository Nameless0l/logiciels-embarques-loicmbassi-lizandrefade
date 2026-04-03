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

const DELAY_MAX_US: u64 = 5_000;
const DELAY_MIN_US: u64 = 200;
const DELAY_RANGE_US: u64 = 4_800;
const MAX_SPEED: i32 = 500;

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

    let mut enc = Encoder::new(
        board.encoder.timer,
        board.encoder.ch_a,
        board.encoder.ch_b,
        board.encoder.button,
    );

    motor.set_microstep(MicrostepMode::Eighth);
    motor.enable();

    let mut speed: i32 = 0;
    let mut prev_pos = enc.position();

    loop {
        // Bouton : arrêt et reset
        if enc.is_pressed() {
            speed = 0;
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
            Timer::after_millis(10).await;
            continue;
        }

        if speed > 0 {
            motor.set_direction(Direction::Clockwise);
        } else {
            motor.set_direction(Direction::CounterClockwise);
        }

        let abs_speed = speed.unsigned_abs().min(MAX_SPEED as u32) as u64;
        let delay = DELAY_MAX_US
            .saturating_sub(DELAY_RANGE_US.saturating_mul(abs_speed) / MAX_SPEED as u64);

        motor.step(delay).await;
    }
}
