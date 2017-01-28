
use std::io;
use std::thread;
use std::cell::RefCell;

use event;

pub struct Interface {
        quit_sig_received: RefCell<bool>
}

impl Interface {
        pub fn new(evt_queue: event::Queue) -> Interface {
                let jh = thread::spawn( move | | event_loop(evt_queue) );
                Interface { quit_sig_received: RefCell::new(false) }
        }

        pub fn supposed_to_quit(&self) -> bool {
                *self.quit_sig_received.borrow()
        }
}

impl event::Handler for Interface {
        fn event(&self, ev: &event::Event) {
                match *ev {
                        event::Event::Quit => *self.quit_sig_received.borrow_mut() = true,
                        _ => {}
                }
        }
}

pub fn event_loop(evt_queue: event::Queue) {
        loop {
                let mut cmd = String::new();
                let l = io::stdin().read_line(&mut cmd);
                match cmd.chars().next() {
                        Some('q') => {
                                evt_queue.pass_event(event::Event::Quit);
                        }
                        _ => {}
                }
        }
}
