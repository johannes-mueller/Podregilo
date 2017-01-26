
use std::io;
use std::thread;

use event;

pub struct Interface {
        join_handle: thread::JoinHandle<()>
}

impl Interface {
        pub fn new(evt_queue: event::Queue) -> Interface {
                let jh = thread::spawn( move | | event_loop(evt_queue) );
                Interface { join_handle: jh }
        }
}

pub fn event_loop(evt_queue: event::Queue) {
        loop {
                let mut cmd = String::new();
                let l = io::stdin().read_line(&mut cmd);
                match cmd.chars().next() {
                        Some('q') => {
                                evt_queue.pass_event(Box::new(QuitEvent));
                        }
                        _ => {}
                }
        }
}


pub struct QuitEvent;

impl event::Event for QuitEvent {
        fn process(&self, dispatcher: &event::Dispatcher) -> bool {
                dispatcher.application_quit();
                false
        }
}
