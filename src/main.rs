use clap::Parser;
use std::{sync::Mutex, thread, sync::Arc};

use std::time::{Instant, Duration};

use log::info;

mod utils;
mod cli;
mod input;
mod watch;

fn main() {
    utils::root_check();

    let config = cli::Config::parse();
    info!("Config: {:?}", config);

    // We use a u8 here instead of a bool to handle the rare case when both shift keys are pressed
    // and then one is released
    let last_received_event: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));
    let lre = Arc::clone(&last_received_event);

    thread::spawn(move || watch::watch_events(lre));

    let mut user_active = true;
    let mut active_since = Instant::now();

    loop {
        let lre = last_received_event.lock().unwrap();
        user_active = if *lre < Instant::now() - Duration::from_secs((config.inactive * 5) as u64) {
            if user_active {
                println!("User just became inactive !");
            }
            false
        } else {
            if !user_active {
                println!("User just became active !");
                active_since = Instant::now();
            }
            true
        };

        let should_pop = active_since < Instant::now() - Duration::from_secs((config.timer * 60) as u64);

        std::mem::drop(lre);
        thread::sleep(Duration::from_millis(500));
    }
}
