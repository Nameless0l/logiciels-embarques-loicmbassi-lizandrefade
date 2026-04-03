#![no_std]
#![no_main]

#[path = "bsp-ensea.rs"]
mod bsp_ensea;
mod gamepad;

use bsp_ensea::Board;
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_time::Timer;
use gamepad::Gamepad;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());
    let board = Board::new(p);

    let pad = Gamepad::new(
        board.gamepad.top,
        board.gamepad.bottom,
        board.gamepad.right,
        board.gamepad.left,
        board.gamepad.center,
    );

    info!("Gamepad ready — appuie sur les boutons");

    loop {
        let s = pad.poll();

        if s.top || s.bottom || s.right || s.left || s.center {
            info!(
                "T:{} B:{} R:{} L:{} C:{}",
                s.top, s.bottom, s.right, s.left, s.center
            );
        }

        Timer::after_millis(50).await;
    }
}
