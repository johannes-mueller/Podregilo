
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use jack_client;

use cli::QuitEvent;

pub trait Event {
        fn process(&self, dispatcher: &Dispatcher) -> bool;
}

pub type EventMsg = Box<Event+Send>;

pub enum UIEvent {
        Quit
}

struct ManagerInternal {
        event_tx: Sender<EventMsg>,
}

pub struct Manager<'a> {
        event_tx_mutex: Arc<Mutex<Sender<EventMsg>>>,
        event_rx: Receiver<EventMsg>,
        dispatcher: Dispatcher<'a>,
}

pub trait Observer<T> {
        fn signal(&self, data: T);
}

pub struct Dispatcher<'a> {
        jingle_observers: Vec<Box<&'a Observer<JingleButtonEvent>>>,
        ui_observers: Vec<Box<&'a Observer<UIEvent>>>
}

impl<'a> Dispatcher<'a> {
        fn new() -> Dispatcher<'a> {
                Dispatcher {
                        jingle_observers: vec![],
                        ui_observers: vec![]
                }
        }

        fn jingle_button_event(&self, ev: JingleButtonEvent) {
                for obs in &self.jingle_observers {
                        obs.signal(ev);
                }
        }

        pub fn register_jingle_observer(&mut self, obs: &'a Observer<JingleButtonEvent>) {
                self.jingle_observers.push(Box::new(obs));
        }

        pub fn application_quit(&self) {
                for obs in &self.ui_observers {
                        obs.signal(UIEvent::Quit);
                }
        }

        pub fn register_ui_observer(&mut self, obs: &'a Observer<UIEvent>) {
                self.ui_observers.push(Box::new(obs));
        }
}

impl<'a> Manager<'a> {
        pub fn new() -> Manager<'a> {
                let (event_tx, event_rx) = mpsc::channel::<EventMsg>();
                Manager {
                        dispatcher: Dispatcher::new(),
                        event_rx: event_rx,
                        event_tx_mutex: Arc::new(Mutex::new(event_tx))
                }
        }
        pub fn event_queue(&self) -> Queue {
                Queue {
                        event_tx_mutex: self.event_tx_mutex.clone()
                }
        }
        pub fn process_next(&self) -> bool {
                let evt = match self.event_rx.recv() {
                        Err(e) => {
                                println!("No event received: {}", e);
                                return true
                        },
                        Ok(ev) => ev
                };

                evt.process(&self.dispatcher)
        }
        pub fn dispatcher(&mut self) -> &mut Dispatcher<'a> {
                &mut self.dispatcher
        }
}

pub struct Queue {
        event_tx_mutex: Arc<Mutex<Sender<EventMsg>>>
}

impl Queue {
        pub fn pass_event(&self, ev: Box<Event+Send>) {
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

pub struct ButtonEvent {
        changed_button: ButtonChange,
}

impl ButtonEvent {
        pub fn new(n: ButtonNumber, s: ButtonState) -> ButtonEvent {
                ButtonEvent { changed_button: (BUTTONS[n], s) }
        }
}

impl Event for ButtonEvent {
        fn process(&self, dispatcher: &Dispatcher) -> bool{
                match self.changed_button {
                        (Button::Jingle(number), ButtonState::Pressed) => {
                                let ev = JingleButtonEvent { number: number, state: ButtonState::Pressed };
                                dispatcher.jingle_button_event(ev);
                        },
                        (Button::Jingle(number), ButtonState::LongPressed) => {
                                let ev = JingleButtonEvent { number: number, state: ButtonState::LongPressed };
                                dispatcher.jingle_button_event(ev);
                        },
                        (Button::Jingle(number), ButtonState::Released)
                                => println!("Released Jingle {}", number),
                        (Button::Rec, ButtonState::Pressed)
                                => println!("Toggled Recenable"),
                        (Button::Mute(channel), ButtonState::Pressed)
                                => println!("Supposed to mute channel {}", channel),
                        (Button::Mute(channel), ButtonState::Released)
                                => println!("Supposed to unmute channel {}", channel),
                        _ => println!("don't know"),
                }

                true
        }
}
