#![allow(dead_code)]

use core::sync::atomic::{AtomicU8, Ordering};

use embassy_stm32::gpio::{AnyPin, Input, Pull};
use embassy_stm32::Peri;

static GAMEPAD_STATE: AtomicU8 = AtomicU8::new(0);

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

    /// Lit l'état du gamepad et met à jour la variable partagée.
    pub fn update_shared_state(&self) {
        let s = self.poll();
        let mut bits = 0u8;
        if s.top { bits |= 1; }
        if s.bottom { bits |= 2; }
        if s.right { bits |= 4; }
        if s.left { bits |= 8; }
        if s.center { bits |= 16; }
        GAMEPAD_STATE.store(bits, Ordering::Relaxed);
    }

    /// Lit l'état partagé. Appelable depuis n'importe quelle tâche.
    pub fn get_shared_state() -> GamepadState {
        let bits = GAMEPAD_STATE.load(Ordering::Relaxed);
        GamepadState {
            top:    bits & 1 != 0,
            bottom: bits & 2 != 0,
            right:  bits & 4 != 0,
            left:   bits & 8 != 0,
            center: bits & 16 != 0,
        }
    }
}
