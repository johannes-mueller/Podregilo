use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

pub trait Event {
        fn process(&self, dispatcher: &Dispatcher);// -> EventResult;
}

pub type EventMsg = Box<Event+Send>;


#[derive(PartialEq)]
pub enum ButtonState {
        Pressed,
        Released
}

pub type ButtonNumber = u8;

pub type ButtonChange = (Button, ButtonState);

#[derive(Copy, Clone)]
pub enum Button {
        Dummy,
        Jingle(usize),
        Rec, Play, AddMark,
        Mute(usize)
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

struct ManagerInternal {
        event_tx: Sender<EventMsg>,
        dispatcher: Dispatcher,
}

pub struct Manager {
        internal: Arc<Mutex<ManagerInternal>>,
        event_rx: Option<Receiver<EventMsg>>
}

struct Dispatcher;

impl Dispatcher {
        fn new() -> Dispatcher {
                Dispatcher
        }

        fn play_jingle(&self, number: usize) {
                println!("Playing jingle {}", number);
        }
}

impl Manager {
        pub fn new() -> Manager {
                let (event_tx, event_rx): (Sender<EventMsg>, Receiver<EventMsg>) = mpsc::channel();
                let internal = Arc::new(Mutex::new(ManagerInternal {
                        dispatcher: Dispatcher::new(),
                        event_tx: event_tx
                }));
                Manager {
                        internal: internal,
                        event_rx: Some(event_rx)
                }
        }
        pub fn clone(&self) -> Manager {
                Manager {
                        internal: self.internal.clone(),
                        event_rx: None
                }
        }
        pub fn button_event(&self, number: usize, state: ButtonState) {
                let evt = Box::new(ButtonEvent {
                        changed_button: (BUTTONS[number], state),
                });
                let intnl = self.internal.lock().unwrap();
                intnl.event_tx.send(evt);
        }
        pub fn process_next(&self) {
                let ref rx = match self.event_rx {
                        None => return,
                        Some(ref rx) => rx
                };
                let evt = match rx.recv() {
                        Err(e) => {
                                println!("No event received: {}", e);
                                return
                        },
                        Ok(ev) => ev
                };

                let intl = self.internal.lock().unwrap();
                evt.process(&intl.dispatcher);
        }
}


impl Event for ButtonEvent {
        fn process(&self, dispatcher: &Dispatcher) {
                match self.changed_button {
                        (Button::Jingle(number), ButtonState::Pressed)
                                => dispatcher.play_jingle(number),
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
        }
}
