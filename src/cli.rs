
use std::io;
use std::thread;

use event;

pub struct Interface {
        join_handle: thread::JoinHandle<()>
}

impl Interface {
        pub fn new(evt_queue: event::Queue) -> Interface {
                let jh = thread::spawn( move | | {
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
                });

                Interface { join_handle: jh }
        }
}

struct QuitEvent;

impl event::Event for QuitEvent {
        fn process(&self, dispatcher: &event::Dispatcher) {
                println!("Supposed to quit");
        }
}
