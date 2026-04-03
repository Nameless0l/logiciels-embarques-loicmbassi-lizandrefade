#![no_main]
#![no_std]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_stm32::gpio::{Input, Pin, Pull};
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

use core::cell::Cell;

static HISTORY_LEN: usize = 10;
static NB_BTN: usize = 5;

static HISTORY_TOP: [Cell<bool>; 10] = [Cell::new(false); 10];
static HISTORY_BOTTOM: [Cell<bool>; 10] = [Cell::new(false); 10];
static HISTORY_RIGHT: [Cell<bool>; 10] = [Cell::new(false); 10];
static HISTORY_LEFT: [Cell<bool>; 10] = [Cell::new(false); 10];
static HISTORY_CENTER: [Cell<bool>; 10] = [Cell::new(false); 10];

static BUTTON_CURRENT_STATE: [Cell<bool>; 5] = [Cell::new(false); 5];
static BUTTON_PREVIOUS_STATE: [Cell<bool>; 5] = [Cell::new(false); 5];

static INDEX: Cell<usize> = Cell::new(0);

//Cell est une astuce permettant de façon safe de modifer une variable déclarée static

pub struct Button<'a> {
    pub id: ButtonId,
    pub pin: &'a Pin<'a, Input<Pull::Up>>,
}

#[derive(Debug)]
struct GamepadState {
    top: bool,
    bottom: bool,
    right: bool,
    left: bool,
    center: bool,
}

#[derive(Copy, Clone)]
pub enum ButtonId {
    Top,
    Bottom,
    Right,
    Left,
    Center,
}

//Donne quel bouton a été appuyé en prenant en compte les rebonds
pub fn push_buttons_state(
    top: &Button,
    bottom: &Button,
    right: &Button,
    left: &Button,
    center: &Button,
) {
    let i = INDEX.get();

    HISTORY_TOP[i].set(top.pin.is_high());
    HISTORY_BOTTOM[i].set(bottom.pin.is_high());
    HISTORY_RIGHT[i].set(right.pin.is_high());
    HISTORY_LEFT[i].set(left.pin.is_high());
    HISTORY_CENTER[i].set(center.pin.is_high());

    INDEX.set((i + 1) % HISTORY_LEN);
}

pub fn all_true_in_list(history: &[Cell<bool>; HISTORY_LEN]) -> bool {
    for i in 0..HISTORY_LEN {
        if history[i].get() != true {
            return false;
        }
    }
    return true;
}

pub fn all_false_in_list(history: &[Cell<bool>; HISTORY_LEN]) -> bool {
    for i in 0..HISTORY_LEN {
        if history[i].get() != false {
            return false;
        }
    }
    return true;
}

pub fn modyfying_current_state() {
    let listof_HISTORIES: [&[Cell<bool>; HISTORY_LEN]; 5] = [
        &HISTORY_TOP,
        &HISTORY_BOTTOM,
        &HISTORY_RIGHT,
        &HISTORY_LEFT,
        &HISTORY_CENTER,
    ]; //Array
    let mut i = 0;
    for x in listof_HISTORIES {
        //&listof_HISTORIES permet de traiter la référence et non une copie
        if (all_true_in_list(x) == true) {
            BUTTON_CURRENT_STATE[i].set(true);
        }
        if (all_false_in_list(x) == true) {
            BUTTON_CURRENT_STATE[i].set(false);
        }
        i += 1;
    }
}

pub fn modifying_previous_state() {
    for i in 0..NB_BTN {
        BUTTON_PREVIOUS_STATE[i].set(BUTTON_CURRENT_STATE[i].get());
    }
}

pub fn impulse_button_changed_state(btn: &Button) -> bool {
    match btn.id {
        ButtonId::Top => BUTTON_PREVIOUS_STATE[0].get() != BUTTON_CURRENT_STATE[0].get(),
        ButtonId::Bottom => BUTTON_PREVIOUS_STATE[1].get() != BUTTON_CURRENT_STATE[1].get(),
        ButtonId::Right => BUTTON_PREVIOUS_STATE[2].get() != BUTTON_CURRENT_STATE[2].get(),
        ButtonId::Left => BUTTON_PREVIOUS_STATE[3].get() != BUTTON_CURRENT_STATE[3].get(),
        ButtonId::Center => BUTTON_PREVIOUS_STATE[4].get() != BUTTON_CURRENT_STATE[4].get(),
    }
}

pub fn is_pressed(btn: &Button) -> bool {
    match btn.id {
        ButtonId::Top => BUTTON_CURRENT_STATE[0].get(),
        ButtonId::Bottom => BUTTON_CURRENT_STATE[1].get(),
        ButtonId::Right => BUTTON_CURRENT_STATE[2].get(),
        ButtonId::Left => BUTTON_CURRENT_STATE[3].get(),
        ButtonId::Center => BUTTON_CURRENT_STATE[4].get(),
    }
}

pub fn button_pool() -> GamepadState {
    GamepadState {
        top: BUTTON_CURRENT_STATE[0].get(),
        bottom: BUTTON_CURRENT_STATE[1].get(),
        right: BUTTON_CURRENT_STATE[2].get(),
        left: BUTTON_CURRENT_STATE[3].get(),
        center: BUTTON_CURRENT_STATE[4].get(),
    }
}
