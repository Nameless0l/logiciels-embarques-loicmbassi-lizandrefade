#![no_std]
#![no_main]

#[path = "bsp-ensea.rs"]
mod bsp_ensea;
mod gamepad_task;

use bsp_ensea::Board;
use core::sync::atomic::{AtomicBool, Ordering};
use defmt::info;
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::Timer;
use gamepad::Gamepad;
use gamepad::GamepadState;
use {defmt_rtt as _, panic_probe as _};

// L'état doit être dans un type thread-safe (Atomic, Mutex, etc.)
static GAMEPAD_STATE: GamepadStateAtomic = GamepadStateAtomic::new();

struct GamepadStateAtomic {
    top: AtomicBool,
    bottom: AtomicBool,
    right: AtomicBool,
    left: AtomicBool,
    center: AtomicBool,
}

impl GamepadStateAtomic {
    pub const fn new() -> Self {
        Self {
            top: AtomicBool::new(false),
            bottom: AtomicBool::new(false),
            right: AtomicBool::new(false),
            left: AtomicBool::new(false),
            center: AtomicBool::new(false),
        }
    }

    pub fn update_if_changed(&self, new: &GamepadState) -> bool {
        let mut changed = false;

        changed |= self.top.swap(new.top, Ordering::Relaxed) != new.top;
        changed |= self.bottom.swap(new.bottom, Ordering::Relaxed) != new.bottom;
        changed |= self.right.swap(new.right, Ordering::Relaxed) != new.right;
        changed |= self.left.swap(new.left, Ordering::Relaxed) != new.left;
        changed |= self.center.swap(new.center, Ordering::Relaxed) != new.center;
        // a = a | b; <=> a |= b
        return changed;
    }
}

#[embassy_executor::task]
async fn gamepad_task(_spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());
    let board = Board::new(p);

    let pad = Gamepad::new(
        board.gamepad.top,
        board.gamepad.bottom,
        board.gamepad.right,
        board.gamepad.left,
        board.gamepad.center,
    );

    info!("Gamepad ready — appuie sur les boutons");

    loop {
        let new_value = pad.poll();

        if GAMEPAD_STATE.update_if_changed(&new_value) {
            info!(
                "T:{} B:{} R:{} L:{} C:{}",
                new_value.top, new_value.bottom, new_value.right, new_value.left, new_value.center
            );
        }
    }
}

//----------------Aefff en dessous
static GAMEPAD_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

async fn wait_and_update() {
    loop {
        GAMEPAD_SIGNAL.wait().await; // wait() ->??Prépare l'attente wait().await -> Dors jusqu'à ce qu'on le réveille

        let new_value = pad.poll(); // Récupère la nouvelle valeur

        update(new_value).await; // Met à jour le Gamepad_State
    }
}

pub fn update(new_value: GAMEPAD_STATE) {
    //doit être appelé depuis une ISR ou autre task
    GAMEPAD_STATE = new_value;
    info!(
        "T:{} B:{} R:{} L:{} C:{}",
        new_value.top, new_value.bottom, new_value.right, new_value.left, new_value.center
    ); //Rappel les envoie de caractères au terminal est très long
}
