
extern crate jack;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::sync::{Arc, RwLock, Mutex};

use std::thread;
use std::time;

use self::jack::JackClient;

use event;

pub type WavData = Vec<[f32; 2]>;

pub type Levels = [f32; 4];

pub type ClipHandle = Arc<RwLock<Option<WavData>>>;


// pub enum ReturnMessage {
//         Ok,
//         NoSuchClip(usize),
//         NotPlaying,
//         AlreadyPlaying,
//         OnPause,
//         NotOnPause,
//         ClipFinished
// }

#[derive(PartialEq, Copy, Clone)]
pub struct ClipPosition {
        clip_number: usize,
        sample_number: usize,
}


#[derive(PartialEq, Copy, Clone)]
pub enum ClientState {
        Idle,
        Playing(ClipPosition),
        Looping(ClipPosition),
        Paused(ClipPosition)
}

#[derive(Copy, Clone)]
pub enum ClientCmd {
        Quit,
        Stop,
        Play(usize),
        ToggleLoop,
        Pause
}

struct Handler {
        clips_handle: Arc<RwLock<Vec<WavData>>>,
        cmd_rx: Receiver<ClientCmd>,
        state_tx: Sender<ClientState>,

        client_state: ClientState,
        buffer_size: usize,
        out_port_l: jack::Port<jack::AudioOutSpec>,
        out_port_r: jack::Port<jack::AudioOutSpec>,

        in_ports: Vec<jack::Port<jack::AudioInSpec>>,

        event_queue: event::Queue,
}

impl Handler {
        fn process_levels(&self, process_scope: &jack::ProcessScope) {
                let mut lev: Levels = [0.0; 4];
                for (mut in_port, max) in self.in_ports.iter().zip(lev.iter_mut()) {
                        let in_p = jack::AudioInPort::new(&in_port, process_scope);
                        *max = in_p.iter().fold(0.0, |m, &x| m.max(x));
                }
                self.event_queue.pass_event(event::Event::Level(lev));
        }
        fn do_process(&mut self, process_scope: &jack::ProcessScope) -> jack::JackControl {
                let mut clip_pos = match self.client_state {
                        ClientState::Idle | ClientState::Paused(_)
                                => return jack::JackControl::Continue,
                        ClientState::Playing(pos) | ClientState::Looping(pos)
                                => pos
                };

                let mut out_l_p = jack::AudioOutPort::new(&mut self.out_port_l, process_scope);
                let mut out_r_p = jack::AudioOutPort::new(&mut self.out_port_r, process_scope);

                let mut out_l = out_l_p.iter_mut();
                let mut out_r = out_r_p.iter_mut();

                let clips = match self.clips_handle.try_read() {
                        Err(_) => panic!("Couldn't get readlock for the clip"),
                        Ok(ch) => ch
                };
                let mut clip_iter = clips[clip_pos.clip_number].iter().skip(clip_pos.sample_number);
                // FIXME: boundary checking in skip()

                loop {
                        let (left,right) = match (out_l.next(), out_r.next()) {
                                (None, None) => break,
                                (Some(l), Some(r)) => (l,r),
                                (_, _) => break
                        };

                        let (l, r) = match clip_iter.next() {
                                None => match self.client_state {
                                        ClientState::Looping(_) => {
                                                clip_iter = clips[clip_pos.clip_number].iter().skip(0);
                                                clip_pos.sample_number = 0;
                                                let s = *clip_iter.next().unwrap();
                                                (s[0], s[1])
                                        },
                                        _ => {
                                                self.client_state = ClientState::Idle;
                                                (0.0, 0.0)
                                        }
                                },
                                Some(s) => (s[0], s[1])
                        };
                        *left = l;
                        *right = r;

                        clip_pos.sample_number += 1;
                }

                self.client_state = match self.client_state {
                        ClientState::Playing(_) => ClientState::Playing(clip_pos),
                        ClientState::Looping(_) => ClientState::Looping(clip_pos),
                        _ => self.client_state
                };

                jack::JackControl::Continue
        }

        fn process_silence(&mut self, process_scope: &jack::ProcessScope) -> jack::JackControl {
                let mut out_l_p = jack::AudioOutPort::new(&mut self.out_port_l, process_scope);
                let mut out_r_p = jack::AudioOutPort::new(&mut self.out_port_r, process_scope);

                let out_l = out_l_p.iter_mut();
                let out_r = out_r_p.iter_mut();

                for (l,r) in out_l.zip(out_r) {
                        *l = 0.0;
                        *r = 0.0;
                };
                jack::JackControl::Continue
        }
}

impl jack::JackHandler for Handler {
        fn process(&mut self, process_scope: &jack::ProcessScope) -> jack::JackControl {
                self.process_levels(process_scope);

                match self.cmd_rx.try_recv() {
                        Err(_) => {},
                        Ok(ClientCmd::Play(cn)) => {
                                let cpos = ClipPosition{ clip_number: cn, sample_number: 0 };
                                self.client_state = ClientState::Playing(cpos);
                        },
                        Ok(ClientCmd::ToggleLoop) => {
                                self.client_state = match self.client_state {
                                        ClientState::Looping(cpos) => ClientState::Playing(cpos),
                                        ClientState::Playing(cpos) => ClientState::Looping(cpos),
                                        _ => self.client_state
                                };
                        },
                        Ok(ClientCmd::Stop) => {
                                self.client_state = ClientState::Idle;
                                return self.process_silence(process_scope);
                        },
                        Ok(ClientCmd::Pause) => {
                                self.client_state = match self.client_state {
                                        ClientState::Playing(pos) |
                                        ClientState::Looping(pos) => ClientState::Paused(pos),
                                        ClientState::Paused(pos) => ClientState::Playing(pos),
                                        ClientState::Idle => ClientState::Idle
                                };
                                match self.client_state {
                                        ClientState::Idle | ClientState::Paused(_) => {
                                                return self.process_silence(process_scope);
                                        },
                                        _ => {}
                                };
                        },
                        Ok(ClientCmd::Quit) => {}
                };

                let ret = self.do_process(process_scope);
                match self.state_tx.send(self.client_state) {
                        Err(e) => panic!("Could not send jack state: {}", e),
                        Ok(_) => {}
                }

                ret
        }

        fn buffer_size(&mut self, buffer_size: jack::JackFrames) -> jack::JackControl {
                self.buffer_size = buffer_size as usize;
                jack::JackControl::Continue
        }

        fn xrun(&mut self) -> jack::JackControl{
                println!("*** xrun ***");
                self.event_queue.pass_event(event::Event::XRun);
                jack::JackControl::Continue
        }
}

struct Manager {
        cmd_rx: Receiver<ClientCmd>,
        jack_cmd_tx: Sender<ClientCmd>,
        state_rx: Receiver<ClientState>,
        state: ClientState,
        attrs: Arc<RwLock<ClientAttrs>>
}

impl Manager {
        fn dispatch_cmd(&mut self, cmd: ClientCmd) {
                match (cmd, self.state) {
                        (ClientCmd::Play(_), ClientState::Playing(_)) => {
                                println!("Already playing");
                        },
                        (ClientCmd::ToggleLoop, ClientState::Idle) |
                        (ClientCmd::ToggleLoop, ClientState::Paused(_)) => {
                                println!("No loop to toggle");
                        },
                        (ClientCmd::Pause, ClientState::Idle) => {
                                println!("Nothing to pause or resume.")
                        }
                        (ClientCmd::Stop, ClientState::Idle) => {
                                println!("Nothing to stop");
                        }
                        (_,_) => {
                                match self.jack_cmd_tx.send(cmd) {
                                        Err(e) => panic!("Could not send ClientCmd: {}", e),
                                        Ok(_) => {}
                                }
                        }
                }
        }

        fn event_loop(&mut self, active_client: &jack::ActiveClient<Handler>) {
                let mut old_time: f32 = 0.0;
                let sample_rate = active_client.sample_rate();
                loop {
                        match self.cmd_rx.try_recv() {
                                Err(_) => {},
                                Ok(ClientCmd::Quit) => { break },
                                Ok(cmd) => { self.dispatch_cmd(cmd) }
                        }
                        let new_state = match self.state_rx.recv() {
                                Err(e) => panic!("Could not receive the sate of JackHandler {}", e),
                                Ok(s) => s
                        };
                        if new_state == self.state {
                                continue;
                        }
                        self.state = new_state;
                        let mut attrs = self.attrs.write()
                                .expect("Could not get write access to ClientState");
                        attrs.state = new_state;

                        match self.state {
                                ClientState::Idle => println!("Idleing"),
                                ClientState::Playing(pos) |
                                ClientState::Looping(pos) => {
                                        let time: f32 = pos.sample_number as f32 / sample_rate as f32;
                                        if (time - old_time).abs() > 1.0 {
                                                println!("Playing {} : {}", pos.clip_number, time);
                                                old_time = time;
                                        }
                                }
                                ClientState::Paused(pos)
                                        => println!("Paused at {} : {}", pos.clip_number, pos.sample_number),
                        }
                }
        }
}

struct ClientAttrs {
        state: ClientState,
        sample_rate: usize
}

pub struct Proxy {
        cmd_tx_mutex: Arc<Mutex<Sender<ClientCmd>>>,
        client_attrs: Arc<RwLock<ClientAttrs>>
}

impl Proxy {
        pub fn new(clips_handle: Arc<RwLock<Vec<WavData>>>,
                   event_queue: event::Queue) -> (Proxy, thread::JoinHandle<()>) {
                let (cmd_tx, cmd_rx) = mpsc::channel::<ClientCmd>();
                let ca = ClientAttrs {
                        state: ClientState::Idle,
                        sample_rate: 0
                };
                let client_attrs = Arc::new(RwLock::new(ca));
                let cattrs = client_attrs.clone();
                let thread_handle = thread::spawn( move | | {
                        register_jack(clips_handle, cmd_rx, cattrs, event_queue);
                });

                let pr = Proxy {
                        cmd_tx_mutex: Arc::new(Mutex::new(cmd_tx)),
                        client_attrs: client_attrs,
                };

                while pr.get_sample_rate() == 0 {
                        thread::sleep(time::Duration::from_millis(100));
                }

                (pr, thread_handle)
        }

        pub fn pass_cmd(&self, cmd: ClientCmd) {
                let cmd_tx = self.cmd_tx_mutex.lock().expect("Could not get command mutex");
                cmd_tx.send(cmd);
        }

        pub fn get_jack_state(&self) -> ClientState {
                let attrs = self.client_attrs.read().expect("Could not get read access to client_attrs.");
                attrs.state
        }

        pub fn get_sample_rate(&self) -> usize {
                let attrs = self.client_attrs.read().expect("Could not get read access to client_attrs.");
                attrs.sample_rate
        }
}


fn register_jack(clips_handle: Arc<RwLock<Vec<WavData>>>,
                 cmd_rx: Receiver<ClientCmd>,
                 client_attrs: Arc<RwLock<ClientAttrs>>,
                 event_queue: event::Queue) {
        let (mut client, _status) = match jack::Client::open("sonbreto", jack::client_options::NO_START_SERVER) {
                Err(_) => panic!("Could not connect to jack."),
                Ok((s,c)) => (s,c)
        };

        client_attrs.write().expect("Could not pass samplerate").sample_rate = client.sample_rate();

        let out_port_l = client.register_port("sonbreto L", jack::AudioOutSpec::default()).unwrap();
        let out_port_r = client.register_port("sonbreto R", jack::AudioOutSpec::default()).unwrap();

        let mut in_ports = vec![];
        for i in 0..4 {
                let s: String = format!("PR vocxo {}", i);
                in_ports.push(client.register_port(&s[..], jack::AudioInSpec::default()).unwrap());
        }

        let (state_tx, state_rx) = mpsc::channel::<ClientState>();
        let (jack_cmd_tx, jack_cmd_rx) = mpsc::channel::<ClientCmd>();

        let jh = Handler {
                clips_handle: clips_handle,
                cmd_rx: jack_cmd_rx,
                state_tx: state_tx,

                client_state: ClientState::Idle,
                buffer_size: client.buffer_size() as usize,
                out_port_l: out_port_l,
                out_port_r: out_port_r,
                in_ports: in_ports,

                event_queue: event_queue
        };

        let active_client = client.activate(jh).unwrap();

        let mut manager = Manager {
                cmd_rx: cmd_rx,
                jack_cmd_tx: jack_cmd_tx,
                state_rx: state_rx,

                state: ClientState::Idle,
                attrs: client_attrs
        };

        manager.event_loop(&active_client);
        println!("Quitting");
        let _ = active_client.deactivate();
}
