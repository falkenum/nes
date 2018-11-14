use ::sdl2::audio::{ AudioSpecDesired, AudioQueue };

pub struct EmulatorAudio {
    queue : AudioQueue<f32>,
}

impl EmulatorAudio {
    pub fn new(sdl_context : &::Sdl) -> EmulatorAudio {
        let spec = AudioSpecDesired {
            freq : None,
            channels : None,
            samples : None,
        };
        EmulatorAudio {
            queue : sdl_context.audio().unwrap().open_queue(None, &spec).unwrap(),
        }
    }
}
