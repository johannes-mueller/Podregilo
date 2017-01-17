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
}

pub struct Manager<'a> {
        event_tx_mutex: Arc<Mutex<Sender<EventMsg>>>,
        event_rx: Receiver<EventMsg>,
        dispatcher: Dispatcher<'a>,
}

pub trait Observer<T> {
        fn signal(&self, data: T);
}

pub enum JingleCmd {
        Stop,
        Play(usize),
        Loop,
        Pause
}

struct Dispatcher<'a> {
        jingle_observers: Vec<Box<&'a Observer<JingleCmd>>>
}

impl<'a> Dispatcher<'a> {
        fn new() -> Dispatcher<'a> {
                Dispatcher {
                        jingle_observers: vec![]
                }
        }

        fn play_jingle(&self, number: usize) {
                for obs in &self.jingle_observers {
                        obs.signal(JingleCmd::Play(number));
                }
        }

        pub fn register_jingle_observer(&mut self, obs: &'a Observer<JingleCmd>) {
                self.jingle_observers.push(Box::new(obs));
        }
}

impl<'a> Manager<'a> {
        pub fn new() -> Manager<'a> {
                let (event_tx, event_rx): (Sender<EventMsg>, Receiver<EventMsg>) = mpsc::channel();
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
        pub fn process_next(&self) {
                let evt = match self.event_rx.recv() {
                        Err(e) => {
                                println!("No event received: {}", e);
                                return
                        },
                        Ok(ev) => ev
                };

                evt.process(&self.dispatcher);
        }
        pub fn dispatcher(&mut self) -> &mut Dispatcher<'a> {
                &mut self.dispatcher
        }
}

pub struct Queue {
        event_tx_mutex: Arc<Mutex<Sender<EventMsg>>>
}

impl Queue {
        pub fn button_event(&self, number: usize, state: ButtonState) {
                let evt = Box::new(ButtonEvent {
                        changed_button: (BUTTONS[number], state),
                });
                let event_tx = self.event_tx_mutex.lock().unwrap();
                event_tx.send(evt);
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
