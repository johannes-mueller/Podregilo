
extern crate serial;
extern crate byteorder;

use std::io::{Read, Write};
use std::thread;
use std::sync::mpsc::{Sender, SyncSender, Receiver};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use self::byteorder::{WriteBytesExt, LittleEndian};

use event;

use jack_client;

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
        msg_tx: SyncSender<Message>,
        level_mutex: Arc<Mutex<jack_client::Levels>>,
        jack_sample_rate: usize,

        record_enabled: Arc<Mutex<bool>>
}

impl Handler {
        pub fn new(port_path: &str,
                   event_queue: event::Queue,
                   jack_sample_rate: usize) -> (Handler, thread::JoinHandle<()>) {
                println!("HAndler");
                let mut port = match serial::open(port_path) {
                        Err(e) => panic!("Could not open serial port: {}", e),
                        Ok(p) => p
                };
                let (msg_tx, msg_rx)= mpsc::sync_channel::<Message>(0);
                let level_mutex = Arc::new(Mutex::new([0.0; 4]));

                let mut conn = Connection::new(port, event_queue, msg_rx, level_mutex.clone());

                let thrd = thread:: spawn( move || { conn.event_loop(); } );

                (Handler { msg_tx: msg_tx,
                           level_mutex: level_mutex,
                           jack_sample_rate: jack_sample_rate,
                           record_enabled: Arc::new(Mutex::new(false))
                }, thrd)
        }

        pub fn show_recenabled(&self, enabled: bool) {
                let mut old = self.record_enabled.lock().expect("Could not get the record enabled lock");
                if enabled == *old {
                        return;
                }
                let msg = Message { head: 'r', data: vec![enabled as u8] };
                self.msg_tx.send(msg);
                *old = enabled;
        }

        pub fn xrun(&self) {
                let msg = Message { head: 'x', data: vec![] };
                self.msg_tx.send(msg);
        }

        fn level(&self, sig: jack_client::Levels) {
                let mut levels = self.level_mutex.lock().expect("Could not get access to level mutex.");
                for (l, s) in levels.iter_mut().zip(&sig) {
                        *l = l.max(*s);
                }
        }

        fn transport_time(&self, ttime: i64) {
                let seconds = (ttime/(self.jack_sample_rate as i64)) as u16;
                let b0 = (seconds & 0b11111111) as u8;
                let b1 = ((seconds >> 8) & 0b11111111) as u8;
                let msg = Message {
                        head: 't',
                        data: vec![b0, b1]
                };
                self.msg_tx.send(msg);
        }

        fn transport_speed(&self, tspeed: f32) {
                let mut msg = Message {
                        head: 's',
                        data: vec![]
                };
                msg.data.write_f32::<LittleEndian>(tspeed).unwrap();
                self.msg_tx.send(msg);
        }
}

impl event::Handler for Handler {
        fn event(&self, ev: &event::Event) {
                match *ev {
                        event::Event::Level(l) => self.level(l),
                        event::Event::ArdourTime(t) => self.transport_time(t),
                        event::Event::ArdourSpeed(s) => self.transport_speed(s),
                        event::Event::RecordEnabled(re) => self.show_recenabled(re),
                        event::Event::XRun => self.xrun(),
                        _ => {}
                };
        }
}

struct Connection {
        port: serial::posix::TTYPort,
        event_queue: event::Queue,
        msg_rx: Receiver<Message>,
        level_mutex: Arc<Mutex<jack_client::Levels>>,
        old_button_state: u16
}

impl Connection {
        fn new(port: serial::posix::TTYPort,
               evt_queue: event::Queue,
               msg_rx: Receiver<Message>,
               level_mutex: Arc<Mutex<jack_client::Levels>>) -> Connection {
                Connection {
                        port: port,
                        event_queue: evt_queue,
                        msg_rx: msg_rx,
                        level_mutex: level_mutex,
                        old_button_state: 0
                }
        }

        fn send_level_msg(&mut self) {
                let mut msg = Message { head: 'l', data: vec![] };
                {
                        let mut levels = self.level_mutex.lock().expect("Could not lock level mutex.");
                        for mut l in &*levels {
                                msg.data.push((l.min(1.0) * 255.0) as u8);
                        }
                        *levels = [0.0; 4];
                }
                self.send_arduino_msg(msg);
        }

        fn send_arduino_msg(&mut self, msg: Message) {
                let mut d = vec![msg.head as u8];
                for b in msg.data {
                        d.push(b);
                }
                self.port.write(&d);
        }

        fn event_loop(&mut self) {
                println!("Connection event loop started");

                let mut buf: [u8; 1] = [0];
                loop {
                        match self.msg_rx.try_recv() {
                                Err(_) => {},
                                Ok(msg) => self.send_arduino_msg(msg)
                        };

                        self.send_level_msg();

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
                                self.event_queue.pass_event(event::button_event(bit, bs));
                        }
                }
        }

        fn long_press_event(&mut self) {
                let long_press_state = self.get_button_state();

                for bit in 0..16 {
                        let mask: u16 = (1 as u16) << bit;
                        if long_press_state & mask != 0 {
                                let ev = event::button_event(bit, event::ButtonState::LongPressed);
                                self.event_queue.pass_event(ev);
                        }
                }
        }
}
