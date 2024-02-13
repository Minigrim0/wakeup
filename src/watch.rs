use std::fs::File;
use std::io::Read;
use std::mem;
use std::{sync::Mutex, sync::Arc};
use std::time::Instant;

use crate::input::{is_key_event, InputEvent};
use crate::utils::get_default_device;

pub fn watch_events(last_received_event: Arc<Mutex<Instant>>) {

    let mut device_file = File::open(&get_default_device()).unwrap_or_else(|e| panic!("{}", e));

    // TODO: use the sizeof function (not available yet) instead of hard-coding 24.
    let mut buf: [u8; 24] = unsafe { mem::zeroed() };

    loop {
        let num_bytes = device_file.read(&mut buf).unwrap_or_else(|e| panic!("{}", e));
        if num_bytes != mem::size_of::<InputEvent>() {
            panic!("Error while reading from device file");
        }
        let event: InputEvent = unsafe { mem::transmute(buf) };
        if is_key_event(event.type_) {
            // Event happened, user is active
            let mut last_received_event = last_received_event.lock().unwrap();
            *last_received_event = Instant::now();
            println!("e");
        }
    }
}