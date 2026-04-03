#![allow(dead_code)]

use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::Peri;
use embassy_time::Timer;

pub enum Direction {
    Clockwise,
    CounterClockwise,
}

pub enum MicrostepMode {
    Full,    // MS1=0, MS2=0
    Half,    // MS1=1, MS2=0
    Quarter, // MS1=0, MS2=1
    Eighth,  // MS1=1, MS2=1
}

pub struct Stepper {
    dir:    Output<'static>,
    ms1:    Output<'static>,
    ms2:    Output<'static>,
    enable: Output<'static>,
    stp:    Output<'static>,
}

impl Stepper {
    pub fn new(
        dir:    Peri<'static, AnyPin>,
        ms1:    Peri<'static, AnyPin>,
        ms2:    Peri<'static, AnyPin>,
        enable: Peri<'static, AnyPin>,
        stp:    Peri<'static, AnyPin>,
    ) -> Self {
        Self {
            dir:    Output::new(dir,    Level::Low,  Speed::Low),
            ms1:    Output::new(ms1,    Level::Low,  Speed::Low),
            ms2:    Output::new(ms2,    Level::Low,  Speed::Low),
            enable: Output::new(enable, Level::High, Speed::Low), // ENN actif bas → démarrage désactivé
            stp:    Output::new(stp,    Level::Low,  Speed::Low),
        }
    }

    pub fn enable(&mut self) {
        self.enable.set_low();
    }

    pub fn disable(&mut self) {
        self.enable.set_high();
    }

    pub fn set_direction(&mut self, dir: Direction) {
        match dir {
            Direction::Clockwise        => self.dir.set_high(),
            Direction::CounterClockwise => self.dir.set_low(),
        }
    }

    pub fn set_microstep(&mut self, mode: MicrostepMode) {
        match mode {
            MicrostepMode::Full    => { self.ms1.set_low();  self.ms2.set_low();  }
            MicrostepMode::Half    => { self.ms1.set_high(); self.ms2.set_low();  }
            MicrostepMode::Quarter => { self.ms1.set_low();  self.ms2.set_high(); }
            MicrostepMode::Eighth  => { self.ms1.set_high(); self.ms2.set_high(); }
        }
    }

    /// Génère une impulsion STEP. `half_period_us` = durée haute = durée basse.
    pub async fn step(&mut self, half_period_us: u64) {
        self.stp.set_high();
        Timer::after_micros(half_period_us).await;
        self.stp.set_low();
        Timer::after_micros(half_period_us).await;
    }

    /// Avance de `steps` pas avec la période donnée.
    pub async fn move_steps(&mut self, steps: u32, half_period_us: u64) {
        for _ in 0..steps {
            self.step(half_period_us).await;
        }
    }
}
