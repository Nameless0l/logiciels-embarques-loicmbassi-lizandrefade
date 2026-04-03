use embassy_stm32::gpio::{AnyPin, Input, Pull};
use embassy_stm32::peripherals::{PA0, PA1, TIM2};
use embassy_stm32::timer::qei::{Config, Qei};
use embassy_stm32::Peri;

const CENTER: u32 = 5_000;
const MAX: u32 = 10_000;

pub struct Encoder {
    qei: Qei<'static, TIM2>,
    button: Input<'static>,
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
        Self { qei, button }
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
        self.button.is_low()
    }
}
