extern crate rodio;

use std::io::BufReader;
use std::fs::File;
use rodio::*;

pub struct Audio {
    sink: rodio::Sink,
}

impl Audio {
    pub fn new(filename: &str) -> Audio {
        let endpoint = rodio::default_endpoint().unwrap();
        let sink = rodio::Sink::new(&endpoint);

        let file = File::open(filename).unwrap();
        let decoder = rodio::Decoder::new(BufReader::new(file)).unwrap();
        let decoder = decoder.repeat_infinite();
        sink.append(decoder);
        Audio { sink: sink }
    }

    pub fn play(&self) {
        self.sink.play()
    }

    pub fn pause(&self) {
        self.sink.pause()
    }
}
