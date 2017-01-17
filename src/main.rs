use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

mod arduino;
mod event;

use event::EventMsg;

fn main() {
        println!("main");

        let event_manager = event::Manager::new();
        let mut arduino = arduino::Handler::new("/dev/ttyUSB0", event_manager.clone());

        arduino.show_recenabled(true);
        loop {
                event_manager.process_next();
        }
}
