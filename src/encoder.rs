use core::sync::atomic::{AtomicI32, Ordering};

use embassy_stm32::Peri;
use embassy_stm32::gpio::{AnyPin, Input, Pull};
use embassy_stm32::peripherals::{PA0, PA1, TIM2};
use embassy_stm32::timer::qei::{Config, Qei};

static ENCODER_POSITION: AtomicI32 = AtomicI32::new(0);

const CENTER: u32 = 5_000;
const MAX: u32 = 10_000;

pub struct Encoder {
    qei: Qei<'static, TIM2>,
    button: Option<Input<'static>>,
}

impl Encoder {
    pub fn new(
        timer: Peri<'static, TIM2>,
        ch_a: Peri<'static, PA0>,
        ch_b: Peri<'static, PA1>,
        button: Peri<'static, AnyPin>,
    ) -> Self {
        let qei = Qei::new(timer, ch_a, ch_b, Config::default());

        let tim2 = embassy_stm32::pac::TIM2;
        tim2.arr().write_value(MAX);
        tim2.cnt().write_value(CENTER);

        let button = Input::new(button, Pull::Up);
        Self {
            qei,
            button: Some(button),
        }
    }

    pub fn new_without_button(
        timer: Peri<'static, TIM2>,
        ch_a: Peri<'static, PA0>,
        ch_b: Peri<'static, PA1>,
    ) -> Self {
        let qei = Qei::new(timer, ch_a, ch_b, Config::default());

        let tim2 = embassy_stm32::pac::TIM2;
        tim2.arr().write_value(MAX);
        tim2.cnt().write_value(CENTER);

        Self { qei, button: None }
    }

    pub fn position(&self) -> i32 {
        (self.qei.count() as i32).wrapping_sub(CENTER as i32)
    }

    pub fn set_position(&mut self, position: i32) {
        let cnt = position.saturating_add(CENTER as i32).max(0) as u32;
        embassy_stm32::pac::TIM2.cnt().write_value(cnt);
    }

    pub fn reset(&mut self) {
        embassy_stm32::pac::TIM2.cnt().write_value(CENTER);
    }

    pub fn is_pressed(&self) -> bool {
        self.button.as_ref().is_some_and(|b| b.is_low())
    }

    /// Met à jour la position partagée. Appelable depuis encoder_task.
    pub fn update_position(pos: i32) {
        ENCODER_POSITION.store(pos, Ordering::Relaxed);
    }

    /// Lit la position partagée. Appelable depuis n'importe quelle tâche.
    pub fn get_position() -> i32 {
        ENCODER_POSITION.load(Ordering::Relaxed)
    }
}
