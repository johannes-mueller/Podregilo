use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

mod arduino;
mod event;

use event::EventMsg;

struct JinglePlayer;

impl event::Observer<event::JingleCmd> for JinglePlayer {
        fn signal(&self, cmd: event::JingleCmd) {
                match cmd {
                        _ => println!("Some JingleCmd")
                }
        }
}

fn main() {
        println!("main");

        let jp = JinglePlayer;
        let mut event_manager = event::Manager::new();
        event_manager.dispatcher().register_jingle_observer(&jp);
        let mut arduino = arduino::Handler::new("/dev/ttyUSB0", event_manager.event_queue());

        arduino.show_recenabled(true);
        loop {
                event_manager.process_next();
        }
}
