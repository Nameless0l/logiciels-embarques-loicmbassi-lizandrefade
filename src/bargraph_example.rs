#![no_std]
#![no_main]

mod bargraph;
#[path = "bsp-ensea.rs"]
mod bsp_ensea;

use bargraph::Bargraph;
use bsp_ensea::Board;
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());
    let board = Board::new(p);

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

    loop {
        let mut v: i32 = 0;
        while v <= 100 {
            bargraph.set_value(v);
            info!("Valeur: {}", v);
            Timer::after_millis(100).await;
            v = v.saturating_add(10);
        }

        let mut v: i32 = 100;
        loop {
            bargraph.set_value(v);
            info!("Valeur: {}", v);
            Timer::after_millis(100).await;
            if v == 0 {
                break;
            }
            v = v.saturating_sub(10);
        }
    }
}
