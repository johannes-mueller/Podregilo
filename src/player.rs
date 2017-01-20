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
}

impl<'a> event::Observer<jack_client::ClientCmd> for JinglePlayer<'a> {
        fn signal(&self, cmd: jack_client::ClientCmd) {
                self.jack_proxy.pass_cmd(cmd)
        }
}
