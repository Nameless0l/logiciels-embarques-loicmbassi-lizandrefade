#![allow(dead_code)]

use embassy_stm32::gpio::{AnyPin, Input, Pull};
use embassy_stm32::Peri;

pub enum Button {
    Top,
    Bottom,
    Right,
    Left,
    Center,
}

pub struct GamepadState {
    pub top: bool,
    pub bottom: bool,
    pub right: bool,
    pub left: bool,
    pub center: bool,
}

pub struct Gamepad {
    top: Input<'static>,
    bottom: Input<'static>,
    right: Input<'static>,
    left: Input<'static>,
    center: Input<'static>,
}

impl Gamepad {
    pub fn new(
        top: Peri<'static, AnyPin>,
        bottom: Peri<'static, AnyPin>,
        right: Peri<'static, AnyPin>,
        left: Peri<'static, AnyPin>,
        center: Peri<'static, AnyPin>,
    ) -> Self {
        Self {
            top: Input::new(top, Pull::Up),
            bottom: Input::new(bottom, Pull::Up),
            right: Input::new(right, Pull::Up),
            left: Input::new(left, Pull::Up),
            center: Input::new(center, Pull::Up),
        }
    }

    pub fn is_pressed(&self, button: &Button) -> bool {
        match button {
            Button::Top => self.top.is_low(),
            Button::Bottom => self.bottom.is_low(),
            Button::Right => self.right.is_low(),
            Button::Left => self.left.is_low(),
            Button::Center => self.center.is_low(),
        }
    }

    pub fn poll(&self) -> GamepadState {
        GamepadState {
            top: self.top.is_low(),
            bottom: self.bottom.is_low(),
            right: self.right.is_low(),
            left: self.left.is_low(),
            center: self.center.is_low(),
        }
    }
}
