#![allow(dead_code)]

use core::sync::atomic::{AtomicU8, AtomicU32, Ordering};

use embassy_stm32::Peri;
use embassy_stm32::gpio::{AnyPin, Level, Output, OutputType, Speed};
use embassy_stm32::peripherals::{PA6, TIM3};
use embassy_stm32::time::hz;
use embassy_stm32::timer::Ch1;
use embassy_stm32::timer::low_level::CountingMode;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;

static STEPPER_SPEED: AtomicU32 = AtomicU32::new(0);
static STEPPER_DIRECTION: AtomicU8 = AtomicU8::new(0);
static STEPPER_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

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
    pwm: SimplePwm<'static, TIM3>,
    dir: Output<'static>,
    ms1: Output<'static>,
    ms2: Output<'static>,
    enable: Output<'static>,
}

impl Stepper {
    pub fn new(
        dir: Peri<'static, AnyPin>,
        ms1: Peri<'static, AnyPin>,
        ms2: Peri<'static, AnyPin>,
        enable: Peri<'static, AnyPin>,
        step: Peri<'static, PA6>,
        timer: Peri<'static, TIM3>,
    ) -> Self {
        let ch1: PwmPin<'static, TIM3, Ch1> = PwmPin::new(step, OutputType::PushPull);
        let pwm = SimplePwm::new(
            timer,
            Some(ch1),
            None,
            None,
            None,
            hz(1_000),
            CountingMode::EdgeAlignedUp,
        );
        Self {
            pwm,
            dir: Output::new(dir, Level::Low, Speed::Low),
            ms1: Output::new(ms1, Level::Low, Speed::Low),
            ms2: Output::new(ms2, Level::Low, Speed::Low),
            enable: Output::new(enable, Level::High, Speed::Low),
        }
    }

    /// Active le driver (ENN actif bas).
    pub fn enable(&mut self) {
        self.enable.set_low();
    }

    /// Désactive le driver et arrête le PWM.
    pub fn disable(&mut self) {
        self.stop();
        self.enable.set_high();
    }

    /// Arrête les impulsions STEP sans couper l'alimentation du driver.
    pub fn stop(&mut self) {
        self.pwm.ch1().disable();
    }

    pub fn set_microstep(&mut self, mode: MicrostepMode) {
        match mode {
            MicrostepMode::Full => {
                self.ms1.set_low();
                self.ms2.set_low();
            }
            MicrostepMode::Half => {
                self.ms1.set_high();
                self.ms2.set_low();
            }
            MicrostepMode::Quarter => {
                self.ms1.set_low();
                self.ms2.set_high();
            }
            MicrostepMode::Eighth => {
                self.ms1.set_high();
                self.ms2.set_high();
            }
        }
    }

    pub fn set_speed(&mut self, steps_per_sec: u32, direction: Direction) {
        match direction {
            Direction::Clockwise => self.dir.set_high(),
            Direction::CounterClockwise => self.dir.set_low(),
        }
        if steps_per_sec == 0 {
            self.stop();
            return;
        }
        self.pwm.set_frequency(hz(steps_per_sec));
        let max = self.pwm.max_duty_cycle();
        self.pwm.ch1().set_duty_cycle(max / 2);
        self.pwm.ch1().enable();
    }

    /// Méthode statique : appelable depuis n'importe quelle tâche.
    pub fn update_speed(speed: u32, direction: Direction) {
        let dir_val = match direction {
            Direction::Clockwise => 0u8,
            Direction::CounterClockwise => 1u8,
        };
        STEPPER_DIRECTION.store(dir_val, Ordering::Relaxed);
        STEPPER_SPEED.store(speed, Ordering::Relaxed);
        STEPPER_SIGNAL.signal(());
    }

    /// Attend une notification, lit la vitesse/direction partagées et met à jour le moteur.
    pub async fn wait_and_update(&mut self) {
        STEPPER_SIGNAL.wait().await;
        let speed = STEPPER_SPEED.load(Ordering::Relaxed);
        let direction = if STEPPER_DIRECTION.load(Ordering::Relaxed) == 0 {
            Direction::Clockwise
        } else {
            Direction::CounterClockwise
        };
        self.set_speed(speed, direction);
    }

    /// Lit la vitesse partagée.
    pub fn get_speed() -> u32 {
        STEPPER_SPEED.load(Ordering::Relaxed)
    }

    /// Lit la direction partagée.
    pub fn get_direction() -> Direction {
        if STEPPER_DIRECTION.load(Ordering::Relaxed) == 0 {
            Direction::Clockwise
        } else {
            Direction::CounterClockwise
        }
    }
}
