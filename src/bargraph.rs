#![allow(dead_code)]

use core::sync::atomic::{AtomicU32, Ordering};

use embassy_stm32::Peri;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use heapless::Vec;

static BARGRAPH_LEVEL: AtomicU32 = AtomicU32::new(0);
static BARGRAPH_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

pub struct Bargraph<const N: usize> {
    leds: Vec<Output<'static>, N>,
    min: i32,
    max: i32,
}

impl<const N: usize> Bargraph<N> {
    pub fn new(pins: [Peri<'static, AnyPin>; N]) -> Self {
        let mut leds: Vec<Output<'static>, N> = Vec::new();
        for pin in pins {
            let _ = leds.push(Output::new(pin, Level::Low, Speed::Low));
        }
        Self {
            leds,
            min: 0,
            max: 100,
        }
    }

    pub fn set_range(&mut self, min: i32, max: i32) {
        self.min = min;
        self.max = max;
    }

    pub fn set_value(&mut self, value: i32) {
        let value = value.clamp(self.min, self.max);
        let range = self.max.saturating_sub(self.min);
        let n = self.leds.len() as i32;
        let lit = if range == 0 {
            0i32
        } else {
            value.saturating_sub(self.min).saturating_mul(n) / range
        };
        for (i, led) in self.leds.iter_mut().enumerate() {
            if (i as i32) < lit {
                led.set_high();
            } else {
                led.set_low();
            }
        }
    }

    pub fn clear(&mut self) {
        for led in self.leds.iter_mut() {
            led.set_low();
        }
    }

    /// Met à jour la valeur partagée et notifie la tâche bargraph.
    pub fn update_value(new_value: u32) {
        BARGRAPH_LEVEL.store(new_value, Ordering::Relaxed);
        BARGRAPH_SIGNAL.signal(());
    }

    pub async fn wait_and_update(&mut self) {
        BARGRAPH_SIGNAL.wait().await;
        let level = BARGRAPH_LEVEL.load(Ordering::Relaxed);
        self.set_value(level as i32);
    }
}
