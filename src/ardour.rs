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
        reply_rx: Receiver<OscMessage>,
}

impl Handler {
        pub fn new(ev_queue: event::Queue) -> (Handler, thread::JoinHandle<()>) {
                let (osc_tx, osc_rx) = mpsc::channel::<OscMessage>();
                let (reply_tx, reply_rx) = mpsc::channel::<OscMessage>();

                let jh = thread::spawn( move | | ardour_osc_loop(osc_rx, reply_tx) );

                let osc_tx_cl = osc_tx.clone();
                let _ = thread::spawn( move | | {
                        loop {
                                thread::sleep(time::Duration::from_millis(100));
                                let msg = osc_message("/transport_frame");
                                if osc_tx_cl.send(msg).is_err() {
                                        break;
                                }
                        }
                });

                osc_tx.send(osc_message("/add_mark"));

                ( Handler { osc_tx: osc_tx, reply_rx: reply_rx }, jh )
        }

        fn add_mark_msg(&self) {
                self.osc_tx.send(OscMessage { addr: "/add_mark".to_string(), args: None });
        }
}

pub fn osc_message(addr: &str) -> OscMessage {
        OscMessage{ addr: addr.to_string(), args: None }
}

fn ardour_osc_loop(osc_rx: Receiver<OscMessage>, reply_tx: Sender<OscMessage>) {
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
        }
}
