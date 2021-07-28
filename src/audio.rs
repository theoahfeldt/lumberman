use std::{fs::File, io::BufReader, time::Duration};

use rodio::{
    source::{Buffered, FadeIn, Repeat},
    Decoder, OutputStream, OutputStreamHandle, Sample, Sink, Source,
};

pub struct AudioResources {
    pub chop: Buffered<Decoder<BufReader<File>>>,
    pub bgm: FadeIn<Repeat<Decoder<BufReader<File>>>>,
}

impl AudioResources {
    pub fn new() -> Self {
        let chop_file = std::fs::File::open("audios/chop.wav").unwrap();
        let chop = Decoder::new(BufReader::new(chop_file)).unwrap().buffered();
        let bgm_file = std::fs::File::open("audios/bird-loop.mp3").unwrap();
        let bgm = Decoder::new(BufReader::new(bgm_file))
            .unwrap()
            .repeat_infinite()
            .fade_in(Duration::from_secs(2));
        Self { chop, bgm }
    }
}

pub struct AudioPlayer {
    volume: f32,
    sinks: Vec<Sink>,
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        Self {
            volume: 1.,
            sinks: vec![],
            _stream,
            stream_handle,
        }
    }

    fn remove_used_sinks(&mut self) {
        self.sinks.retain(|s| !Sink::empty(s));
    }

    pub fn play<S>(&mut self, source: S)
    where
        S: Source + Send + 'static,
        S::Item: Sample,
        S::Item: Send,
    {
        let sink = Sink::try_new(&self.stream_handle).unwrap();
        sink.set_volume(self.volume);
        sink.append(source);
        self.sinks.push(sink);
        self.remove_used_sinks();
    }
}
