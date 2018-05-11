use ::sdl2::{ render, pixels, video };
use self::pixels::PixelFormatEnum;
// use self::event::Event;
// use self::keyboard::Keycode;
// use std::time::Duration;

const WIDTH : usize = 256;
const HEIGHT : usize = 240;
const BYTES_PER_PIXEL : usize = 3;
const SCREEN_SIZE : usize = WIDTH * HEIGHT * BYTES_PER_PIXEL;
const FORMAT : PixelFormatEnum = PixelFormatEnum::BGR24;

pub struct Screen {
    canvas : render::Canvas<video::Window>,
    tc : render::TextureCreator<video::WindowContext>,
}

pub struct Picture<'a> {
    texture : render::Texture<'a>
}

impl<'a> Screen {
    pub fn new() -> Screen {
        let sdl_context = ::sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("first", 400, 400)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        let tc = canvas.texture_creator();


        Screen {
            canvas : canvas,
            tc : tc,
        }
    }

    fn update(&mut self, picture : &render::Texture) {
        self.canvas.copy(&picture, None, None).unwrap();
        self.canvas.present();
    }

    pub fn new_picture(&'a mut self) -> Picture<'a> {
        Picture {
            texture : self.tc.create_texture_streaming(FORMAT, WIDTH as u32,
                                                   HEIGHT as u32).unwrap(),
        }
    }
}

impl<'a> Picture<'a> {
    pub fn update(&mut self, pixeldata : &[u8]) {
        self.texture.update(None, &pixeldata, WIDTH*BYTES_PER_PIXEL).unwrap();
    }
}

// let mut event_pump = sdl_context.event_pump().unwrap();
// 'running: loop {
//     for event in event_pump.poll_iter() {
//         match event {
//             Event::Quit {..} | Event::KeyDown {..} => break 'running,
//             _ => {},
//         }
//     }

//     ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
// }
