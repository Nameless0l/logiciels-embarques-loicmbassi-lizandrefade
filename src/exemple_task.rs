#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};
static BARGRAPH_LEVEL: AtomicU32 = AtomicU32::new(0);
static BARGRAPH_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // On spawn une tâche
    spawner.spawn(blink_task()).unwrap();

    loop {
        defmt::info!("Main loop");
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[embassy_executor::task]
async fn blink_task() {
    loop {
        defmt::info!("Blink!");
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task]
async fn wait_and_update() {
    loop {
        // Attend qu'un update soit demandé
        BARGRAPH_SIGNAL.wait().await; // wait() ->??Prépare l'attente wait().await -> Dors jusqu'à ce qu'on le réveille

        // Récupère la nouvelle valeur
        let new_value = BARGRAPH_LEVEL.load(Ordering::Relaxed);

        // Met à jour le bargraph
        refresh_bargraph(new_value).await;
    }
}

pub fn update(new_value: u32) {
    //doit être appelé depuis une ISR ou autre task
    BARGRAPH_LEVEL.store(new_value, Ordering::Relaxed);

    // Réveille la task
    BARGRAPH_SIGNAL.signal(()); //Signal transporte aucune données
}
