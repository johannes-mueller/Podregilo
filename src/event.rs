
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use jack_client;

pub enum Event {
        Quit,
        JingleButton(JingleButtonEvent),
//        Button(ButtonChange),
        Level(jack_client::Levels),
        ArdourTime(i64),
        Dummy
}

struct ManagerInternal {
        event_tx: Sender<Event>,
}

pub struct Manager<'a> {
        event_rx: Receiver<Event>,
        handlers: Vec<Box<&'a Handler>>
}

pub trait Handler {
        fn event(&self, ev: &Event);
}

impl<'a> Manager<'a> {
        pub fn new(ev_rx: Receiver<Event>) -> Manager<'a> {
                Manager {
                        handlers: vec![],
                        event_rx: ev_rx
                }
        }
        pub fn process_next(&self) {
                let ev = match self.event_rx.recv() {
                        Err(e) => {
                                println!("No event received: {}", e);
                                return
                        },
                        Ok(ev) => ev
                };
                self.signal_event(&ev);
        }
        pub fn register_event_handler(&mut self, handler: &'a Handler) {
                self.handlers.push(Box::new(handler));
        }
        pub fn signal_event(&self, ev: &'a Event) {
                for h in &self.handlers {
                        h.event(ev);
                }
        }
}

#[derive(Clone)]
pub struct Queue {
        event_tx_mutex: Arc<Mutex<Sender<Event>>>
}

impl Queue {
        pub fn new(ev_tx: Sender<Event>) -> Queue {
                Queue {
                        event_tx_mutex: Arc::new(Mutex::new(ev_tx))
                }
        }
        pub fn pass_event(&self, ev: Event) {
                let event_tx = self.event_tx_mutex.lock().unwrap();
                event_tx.send(ev);
        }
}


#[derive(PartialEq, Copy, Clone)]
pub enum ButtonState {
        Pressed,
        Released,
        LongPressed
}

pub type ButtonNumber = usize;

pub type ButtonChange = (Button, ButtonState);

#[derive(Copy, Clone)]
pub enum Button {
        Dummy,
        Jingle(usize),
        Rec, Play, AddMark,
        Mute(usize)
}


#[derive(Copy, Clone)]
pub struct JingleButtonEvent {
        pub number: usize,
        pub state: ButtonState
}

const BUTTONS: [Button; 16] = [
        Button::Mute(0), Button::Mute(1), Button::Mute(2), Button::Mute(3),
        Button::Dummy, Button::Dummy, Button::Dummy, Button::Dummy,
        Button::Jingle(0), Button::Jingle(1), Button::Jingle(2), Button::Jingle(3),
        Button::Rec, Button::Play, Button::AddMark, Button::Dummy
];

pub fn button_event(number: usize, state: ButtonState) -> Event {
        let button = BUTTONS[number];

        match button {
                Button::Jingle(n) => Event::JingleButton(JingleButtonEvent { number: n, state: state }),
                 _ => Event::Dummy
        }
}
