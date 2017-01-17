
extern crate serial;

use std::io::{Read, Write};
use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use event;
use event::EventMsg;

const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate:    serial::Baud9600,
    char_size:    serial::Bits8,
    parity:       serial::ParityNone,
    stop_bits:    serial::Stop1,
    flow_control: serial::FlowNone
};


struct Message {
        head: char,
        data: Vec<u8>,
}

pub struct Handler {
        msg_tx: Sender<Message>,
        join_handle: thread::JoinHandle<()>
}

impl Handler {
        pub fn new(port_path: &str, event_manager: event::Manager) -> Handler {
                println!("HAndler");
                let mut port = match serial::open("/dev/ttyUSB0") {
                        Err(e) => panic!("Could not open serial port: {}", e),
                        Ok(p) => p
                };
                let (msg_tx, msg_rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
                let mut conn = Connection::new(port, event_manager, msg_rx);

                let thrd = thread:: spawn( move || { conn.event_loop(); } );

                Handler { msg_tx: msg_tx, join_handle: thrd }
        }

        pub fn show_recenabled(&self, enabled: bool) {
                let msg = Message { head: 'r', data: vec![enabled as u8] };
                self.msg_tx.send(msg);
        }
}

struct Connection {
        port: serial::posix::TTYPort,
        event_manager: event::Manager,
        msg_rx: Receiver<Message>,
        old_button_state: u16
}

impl Connection {
        fn new(port: serial::posix::TTYPort,
               evt_mngr: event::Manager, msg_rx: Receiver<Message>) -> Connection {
                Connection {
                        port: port,
                        event_manager: evt_mngr,
                        msg_rx: msg_rx,
                        old_button_state: 0
                }
        }

        fn event_loop(&mut self) {
                println!("Connection event loop started");


                let mut buf: [u8; 1] = [0];
                loop {
                        match self.msg_rx.try_recv() {
                                Err(_) => {},
                                Ok(msg) => {
                                        println!("received {} {}", msg.head, msg.data[0]);
                                        let mut d = vec![msg.head as u8];
                                        for b in msg.data {
                                                d.push(b);
                                        }
                                        println!("received {} {}", d[0] as char, d[1]);
                                        self.port.write(&d);
                                }
                        };

                        match self.port.read_exact(&mut buf) {
                                Err(_) => continue,
                                Ok(_) => {}
                        };

                        match buf[0] {
                                b'?' => self.answer_probe(),
                                b'b' => self.button_event(),
                                b => println!("Unknown signal identifier byte: {}", b as u8)
                        }
                }
        }

        fn answer_probe(&mut self) {
                println!("Answering probe");
                self.port.write(b"!");
        }

        fn button_event(&mut self) {
                let mut buf: [u8; 2] = [0; 2];
                self.port.read_exact(&mut buf).unwrap();

                let button_state: u16 = ((buf[0] as u16)<< 8) + buf[1] as u16;
                let changed_button: u16 = button_state ^ self.old_button_state;
                self.old_button_state = button_state;

                let mut mask: u16 = 1;
                for bit in 0..16 {
                        let mask: u16 = (1 as u16) << bit;
                        if changed_button & mask != 0 {
                                let bs: event::ButtonState = match button_state & mask {
                                        0 => event::ButtonState::Released,
                                        _ => event::ButtonState::Pressed
                                };
                                let evt = Box::new(self.event_manager.button_event(bit, bs));
                        }
                }
        }
}