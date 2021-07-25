use image::{imageops, io::Reader};
use luminance::context::GraphicsContext;
use luminance_front::Backend;

use crate::object::{ResourceManager, TextureResource};
use std::time::{Duration, Instant};

pub struct Frame {
    texture: TextureResource,
    duration: Duration,
}

pub struct Animation {
    frames: Vec<Frame>,
    start_time: Instant,
    looping: bool,
}

impl Animation {
    pub fn new(frames: Vec<Frame>) -> Self {
        Self {
            frames,
            start_time: Instant::now(),
            looping: false,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Instant::now();
    }

    pub fn start_loop(&mut self) {
        self.start_time = Instant::now();
        self.looping = true;
    }

    pub fn get_current_texture(&self) -> TextureResource {
        let mut elapsed = self.start_time.elapsed();

        if self.looping {
            let total_duration: Duration = self.frames.iter().map(|f| f.duration).sum();
            let loops = (elapsed.as_secs_f32() / total_duration.as_secs_f32()) as u32;
            elapsed -= total_duration * loops;
        }

        let mut sum_duration = Duration::ZERO;
        let mut texture = self.frames.last().unwrap().texture;
        for f in &self.frames {
            sum_duration += f.duration;
            if elapsed < sum_duration {
                texture = f.texture;
                break;
            }
        }
        texture
    }
}

pub struct GameAnimations {
    pub chop: Animation,
}

impl GameAnimations {
    pub fn new(
        rm: &mut ResourceManager,
        ctxt: &mut impl GraphicsContext<Backend = Backend>,
    ) -> Self {
        let frame1 = Reader::open("textures/lumberjack-chop-1.png")
            .unwrap()
            .decode()
            .unwrap()
            .into_rgba8();
        let frame2 = Reader::open("textures/lumberjack-chop-2.png")
            .unwrap()
            .decode()
            .unwrap()
            .into_rgba8();
        let frame1 = rm.make_texture(ctxt, &imageops::flip_vertical(&frame1));
        let frame2 = rm.make_texture(ctxt, &imageops::flip_vertical(&frame2));
        Self {
            chop: Animation::new(vec![
                Frame {
                    texture: frame1,
                    duration: Duration::from_millis(50),
                },
                Frame {
                    texture: frame2,
                    duration: Duration::from_millis(200),
                },
                Frame {
                    texture: frame1,
                    duration: Duration::from_millis(100),
                },
            ]),
        }
    }

    pub fn update(&mut self) {
        self.chop.start();
    }
}
