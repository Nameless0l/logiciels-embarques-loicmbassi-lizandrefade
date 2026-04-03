#![no_main]
#![no_std]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _p: embassy_stm32::Peripherals = embassy_stm32::init(Config::default());

    loop {
        info!("Hello World!");
    }
}
/*
let p = embassy_stm32::init(Config::default());
let board = Board::new(p);
 Bargraph::new(board.bargraph), Gamepad::new(board.gamepad), etc.

*/
