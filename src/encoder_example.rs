#![no_std]
#![no_main]

#[path = "bsp-ensea.rs"]
mod bsp_ensea;
mod encoder;

use bsp_ensea::Board;
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_time::Timer;
use encoder::Encoder;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());
    let board = Board::new(p);

    let mut enc = Encoder::new(
        board.encoder.timer,
        board.encoder.ch_a,
        board.encoder.ch_b,
        board.encoder.button.into(),
    );

    loop {
        let pos = enc.position();
        let pressed = enc.is_pressed();
        info!("Position: {}, Bouton: {}", pos, pressed);

        if pressed {
            enc.reset();
            info!("Reset!");
        }

        Timer::after_millis(500).await;
    }
}
