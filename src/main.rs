extern crate argparse;
use argparse::{ArgumentParser, List};

extern crate wavefile;
use wavefile::WaveFile;

extern crate itertools;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::sync::{Arc, Mutex, RwLock};

mod arduino;
mod ardour;
mod event;
mod player;
mod cli;
mod jack_client;


fn fill_clips_handle(infiles: Vec<String>, clips_handle: Arc<RwLock<Vec<jack_client::WavData>>>) {
        let mut clips = clips_handle.write().unwrap();

        for (i, filename) in itertools::enumerate(&infiles) {
                println!("Opening file {}: {}", i, filename);
                let wavefile = match WaveFile::open(filename.to_string()) {
                        Ok(w) => w,
                        Err(e) => panic!("{}", e)
                };

                let mut wavdata = vec![];
                for frame in wavefile.iter() {
                        wavdata.push([frame[0] as f32 / 20000., frame[1] as f32 / 20000. ]);
                }
                clips.push(wavdata);
        }
}

fn parse_args(infiles: &mut Vec<String>) {
        let mut ap = ArgumentParser::new();

        ap.set_description("Test soundboard ...");
        ap.refer(infiles).add_argument("input file", List, "WAV file to test").required();
        ap.parse_args_or_exit();
}


fn main() {
        let clips_handle = Arc::new(RwLock::new(vec![]));
        let mut infiles: Vec<String> = vec![];
        parse_args(&mut infiles);
        fill_clips_handle(infiles, clips_handle.clone());

        let (event_tx, event_rx) = mpsc::channel::<event::Event>();
        let ev_queue = event::Queue::new(event_tx);

        let (jack_proxy,  jack_thread) = jack_client::Proxy::new(clips_handle.clone(), ev_queue.clone());
        let jp = player::JinglePlayer::new(&jack_proxy);
        let (arduino, arduino_thread) = arduino::Handler::new("/dev/ttyUSB0", ev_queue.clone());
        let (ardour, ardour_thread) = ardour::Handler::new(ev_queue.clone());
        let cli = cli::Interface::new(ev_queue.clone());

        let mut event_manager = event::Manager::new(event_rx);

        event_manager.register_event_handler(&jp);
        event_manager.register_event_handler(&arduino);
        event_manager.register_event_handler(&cli);

        arduino.show_recenabled(true);
        while !cli.supposed_to_quit() {
                event_manager.process_next();
        }

        let _ = jack_thread.join();
}
