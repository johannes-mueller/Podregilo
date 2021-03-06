use std::sync::mpsc::{Sender,Receiver};
use std::sync::mpsc;

use event;
use jack_client;

enum ButtonRelease {
        Released
}


pub struct JinglePlayer<'a> {
        jack_proxy: &'a jack_client::Proxy
}

impl<'a> JinglePlayer<'a> {
        pub fn new(jp: &jack_client::Proxy) -> JinglePlayer {
                JinglePlayer {
                        jack_proxy: jp
                }
        }

        fn button_press(&self, number: usize) {
                let cmd = match self.jack_proxy.get_jack_state() {
                        jack_client::ClientState::Idle => jack_client::ClientCmd::Play(number),
                        jack_client::ClientState::Playing(_) |
                        jack_client::ClientState::Paused(_) => jack_client::ClientCmd::Pause,
                        jack_client::ClientState::Looping(_) => jack_client::ClientCmd::ToggleLoop,
                };
                self.jack_proxy.pass_cmd(cmd);
        }

        fn button_long_pressed(&self, number: usize) {
                println!("Long press: {}", number);
                let cmd = match self.jack_proxy.get_jack_state() {
                        jack_client::ClientState::Playing(_) => jack_client::ClientCmd::ToggleLoop,
                        jack_client::ClientState::Paused(_) => jack_client::ClientCmd::Stop,
                        _ => return
                };
                self.jack_proxy.pass_cmd(cmd)
        }

        fn jingle_button_event(&self, ev: event::JingleButtonEvent) {
                match ev.state {
                        event::ButtonState::Pressed => self.button_press(ev.number),
                        event::ButtonState::LongPressed => self.button_long_pressed(ev.number),
                        _ => {}
                };
        }
}

impl<'a> event::Handler for JinglePlayer<'a> {
        fn event(&self, ev: &event::Event) {
                match *ev {
                        event::Event::Quit => self.jack_proxy.pass_cmd(jack_client::ClientCmd::Quit),
                        event::Event::JingleButton(e) => self.jingle_button_event(e),
                        _ => {}
                };
        }
}
