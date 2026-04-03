#![no_std]
#![no_main]

#[path = "bsp-ensea.rs"]
mod bsp_ensea;
mod gamepad;

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

pub struct Button<'a> {
    pub id: ButtonId,
    pub pin: &'a Pin<'a, _, Input<PullUp>>,
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());
    let btn_top    = Button { id: ButtonId::Top,    pin: &p.PC8 };
    let btn_bottom = Button { id: ButtonId::Bottom, pin: &p.PB11 };
    let btn_right  = Button { id: ButtonId::Right,  pin: &p.PC9 };
    let btn_left   = Button { id: ButtonId::Left,   pin: &p.PC6 };
    let btn_center = Button { id: ButtonId::Center, pin: &p.PC5 };
    let list_button =[btn_top,btn_bottom,btn_right,btn_left,btn_center]
    let list_button_name =["btn_top","btn_bottom","btn_right","btn_left","btn_center"]

    loop {
        push_buttons_state(top, bottom, right, left, center);
        modyfying_current_state();
        for x in list_button {
            if impulse_button_changed_state(x) {
                
            }
        }
        let pos = enc.position();
        let pressed = enc.is_pressed();
        info!("Position: {}, Bouton: {}", pos, pressed);

        Timer::after_millis(20).await; //Si pas await la tâche ne se fai