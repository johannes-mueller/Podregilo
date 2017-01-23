
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
}

impl Handler {
        pub fn new(port_path: &str, event_queue: event::Queue) -> (Handler, thread::JoinHandle<()>) {
                println!("HAndler");
                let mut port = match serial::open("/dev/ttyUSB0") {
                        Err(e) => panic!("Could not open serial port: {}", e),
                        Ok(p) => p
                };
                let (msg_tx, msg_rx)= mpsc::channel::<Message>();
                let mut conn = Connection::new(port, event_queue, msg_rx);

                let thrd = thread:: spawn( move || { conn.event_loop(); } );

                (Handler { msg_tx: msg_tx }, thrd)
        }

        pub fn show_recenabled(&self, enabled: bool) {
                let msg = Message { head: 'r', data: vec![enabled as u8] };
                self.msg_tx.send(msg);
        }
}

struct Connection {
        port: serial::posix::TTYPort,
        event_queue: event::Queue,
        msg_rx: Receiver<Message>,
        old_button_state: u16
}

impl Connection {
        fn new(port: serial::posix::TTYPort,
               evt_queue: event::Queue, msg_rx: Receiver<Message>) -> Connection {
                Connection {
                        port: port,
                        event_queue: evt_queue,
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
                                b'l' => self.long_press_event(),
                                b => println!("Unknown signal identifier byte: {}", b as u8)
                        }
                }
        }

        fn answer_probe(&mut self) {
                println!("Answering probe");
                self.port.write(b"!");
        }

        fn get_button_state(&mut self) -> u16 {
                let mut buf: [u8; 2] = [0; 2];
                self.port.read_exact(&mut buf).unwrap();

                ((buf[0] as u16)<< 8) + buf[1] as u16
        }

        fn button_event(&mut self) {
                let button_state = self.get_button_state();
                let changed_button: u16 = button_state ^ self.old_button_state;
                self.old_button_state = button_state;

                for bit in 0..16 {
                        let mask: u16 = (1 as u16) << bit;
                        if changed_button & mask != 0 {
                                let bs: event::ButtonState = match button_state & mask {
                                        0 => event::ButtonState::Released,
                                        _ => event::ButtonState::Pressed
                                };
                                let ev = Box::new(event::ButtonEvent::new(bit, bs));
                                self.event_queue.pass_event(ev);
                        }
                }
        }

        fn long_press_event(&mut self) {
                let long_press_state = self.get_button_state();

                for bit in 0..16 {
                        let mask: u16 = (1 as u16) << bit;
                        if long_press_state & mask != 0 {
                                let ev = Box::new(event::ButtonEvent::new(bit, event::ButtonState::LongPressed));
                                self.event_queue.pass_event(ev);
                        }
                }
        }
}
