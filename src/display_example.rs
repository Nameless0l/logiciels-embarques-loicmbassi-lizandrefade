#![no_std]
#![no_main]

#[path = "bsp-ensea.rs"]
mod bsp_ensea;

use bsp_ensea::Board;
use core::fmt::Write;
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_stm32::i2c::I2c;
use embassy_time::Timer;
use embedded_graphics::{
    mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use heapless::String;
use ssd1306::{I2CDisplayInterface, Ssd1306, prelude::*};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());
    let board = Board::new(p);

    let i2c = I2c::new_blocking(
        board.i2c1.i2c,
        board.i2c1.scl,
        board.i2c1.sda,
        Default::default(),
    );

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    let mut counter: u32 = 0;

    loop {
        display.clear_buffer();

        Text::with_baseline("Hello ENSEA!", Point::new(0, 0), style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        let mut buf: String<32> = String::new();
        write!(buf, "Count: {}", counter).ok();
        Text::with_baseline(&buf, Point::new(0, 16), style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        display.flush().unwrap();

        info!("Count: {}", counter);
        counter = counter.saturating_add(1);
        Timer::after_millis(500).await;
    }
}
