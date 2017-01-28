extern crate rosc;

use std::thread;
use std::sync::mpsc::{Sender, SyncSender, Receiver};
use std::sync::mpsc;
use std::net;

use std::time;

use self::rosc::{OscPacket, OscMessage, OscType};

use event;

pub struct Handler {
        osc_tx: Sender<OscMessage>,
}

impl Handler {
        pub fn new(ev_queue: event::Queue) -> (Handler, thread::JoinHandle<()>) {
                let (osc_tx, osc_rx) = mpsc::channel::<OscMessage>();
                let (reply_tx, reply_rx) = mpsc::channel::<OscPacket>();

                let jh = thread::spawn( move | | ardour_osc_loop(osc_rx, reply_tx) );

                let osc_tx_cl = osc_tx.clone();
                let _ = thread::spawn( move | | {
                        loop {
                                thread::sleep(time::Duration::from_millis(100));
                                if osc_tx_cl.send(osc_message("/transport_frame")).is_err() {
                                        break;
                                }
                                if osc_tx_cl.send(osc_message("/record_enabled")).is_err() {
                                        break;
                                }
                        }
                });

                let _ = thread::spawn( move | | {
                        loop {
                                let reply = match reply_rx.recv() {
                                        Err(_) => {
                                                println!("Error receiving reply");
                                                continue;
                                        },
                                        Ok(r) => r
                                };
                                handle_reply(&ev_queue, &reply);
                        }
                });

                osc_tx.send(osc_message("/add_mark"));

                ( Handler { osc_tx: osc_tx }, jh )
        }

        fn add_mark_msg(&self) {
                self.osc_tx.send(OscMessage { addr: "/add_mark".to_string(), args: None });
        }
}

fn handle_reply(ev_queue: &event::Queue, osc_reply: &OscPacket) {
        match osc_reply {
                &OscPacket::Bundle(ref bundle) => {
                        for p in &bundle.content {
                                handle_reply(ev_queue, &p);
                        };
                },
                &OscPacket::Message(ref msg) => handle_osc_message(ev_queue, &msg)
        }
}

macro_rules! extract_osc {
        ($msg:ident, $variant:ident) => {{
                let args = match $msg.args {
                        None => {
                                println!("expected content with Osc Message");
                                return;
                        },
                        Some(ref v) => v
                };
                let val = match args[0] {
                        OscType::$variant(val) => val,
                        _ => {
                                println!("Wrong type received in Osc message");
                                return;
                        }
                };
                val
        }}
}

fn handle_osc_message(ev_queue: &event::Queue, msg: &OscMessage) {
        match msg.addr.as_ref() {
                "/record_enabled" =>
                        ev_queue.pass_event(event::Event::RecordEnabled(extract_osc!(msg, Int) != 0)),
                "/transport_frame" =>
                        ev_queue.pass_event(event::Event::ArdourTime(extract_osc!(msg, Long))),
                _ => println!("Received OSC message {}", msg.addr)
        }
}

pub fn osc_message(addr: &str) -> OscMessage {
        OscMessage{ addr: addr.to_string(), args: None }
}

fn ardour_osc_loop(osc_rx: Receiver<OscMessage>, reply_tx: Sender<OscPacket>) {
        let addr = ("127.0.0.1", 3819);
        let mut sock = net::UdpSocket::bind("127.0.0.1:12345").expect("Could not setup UdpSocket");
        let _ = sock.set_nonblocking(true);
        let mut buf = [0u8; rosc::decoder::MTU];

        loop {
                let osc_msg = match osc_rx.recv() {
                        Ok(msg) => msg,
                        Err(_) => continue,
                };

                let sbuf = match rosc::encoder::encode(&OscPacket::Message(osc_msg)) {
                        Ok(b) => b,
                        Err(_) => {
                                println!("Invalid OscPacket");
                                continue;
                        }
                };

                sock.send_to(&sbuf, addr).unwrap_or_else( |e| { println!("Could not send Osc message:"); 0} );

                if sock.recv_from(&mut buf).is_err() {
                        continue
                }

//                println!("Osc reply {}", String::from_utf8_lossy(&buf));
                let reply_msg = match rosc::decoder::decode(&buf) {
                        Ok(msg) => msg,
                        Err(_) => {
                                println!("Invalid osc reply: {}", String::from_utf8_lossy(&buf));
                                continue
                        }
                };

                reply_tx.send(reply_msg);
        }
}
