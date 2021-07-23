use std::{fs::File, io::BufReader};

use rodio::{source::Buffered, Decoder, OutputStream, OutputStreamHandle, Sample, Sink, Source};

pub struct AudioResources {
    pub chop: Buffered<Decoder<BufReader<File>>>,
}

impl AudioResources {
    pub fn new() -> Self {
        let file = std::fs::File::open("audios/chop.wav").unwrap();
        let chop = Decoder::new(BufReader::new(file)).unwrap().buffered();
        Self { chop }
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
