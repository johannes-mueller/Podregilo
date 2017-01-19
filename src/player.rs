use std::sync::mpsc::{Sender,Receiver};
use std::sync::mpsc;

use event;

enum ButtonRelease {
        Released
}

pub enum JingleCmd {
        Stop,
        Play(usize),
        Loop,
        Pause
}

pub struct JinglePlayer {
        release_tx: Option<Sender<ButtonRelease>>,
}

impl JinglePlayer {
        pub fn new() -> JinglePlayer {
                JinglePlayer {
                        release_tx: None
                }
        }
}

impl event::Observer<JingleCmd> for JinglePlayer {
        fn signal(&self, cmd: JingleCmd) {
                match cmd {
                        _ => println!("Some JingleCmd")
                }
        }
}
