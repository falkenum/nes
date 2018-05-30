use ::sdl2::{ Sdl, VideoSubsystem, render, pixels, video };
use self::pixels::PixelFormatEnum;
use super::input::EmulatorInput;

const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT : u32 = 800;

const FORMAT : PixelFormatEnum = PixelFormatEnum::BGR24;
const WIDTH : usize = 256;
const HEIGHT : usize = 240;
const BYTES_PER_PIXEL : usize = 3;
pub const SCREEN_SIZE : usize = WIDTH * HEIGHT * BYTES_PER_PIXEL;

pub struct Screen {
    sdl_context : Sdl,
    video_subsystem : VideoSubsystem,
    canvas : render::Canvas<video::Window>,
}

pub struct PictureCreator {
    texture_creator : render::TextureCreator<video::WindowContext>,
}

pub struct Picture<'a> {
    texture : render::Texture<'a>
}

impl Screen {
    pub fn new() -> Screen {
        let sdl_context = ::sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("NES", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Screen {
            sdl_context : sdl_context,
            video_subsystem : video_subsystem,
            canvas : canvas,
        }
    }

    pub fn update_and_show(&mut self, picture : &Picture) {
        self.canvas.copy(&picture.texture, None, None).unwrap();
        self.canvas.present();
    }

    pub fn picture_creator(&self) -> PictureCreator {
        PictureCreator {
            texture_creator : self.canvas.texture_creator()
        }
    }

    pub fn emulator_input(&self) -> EmulatorInput {
        EmulatorInput::new(self.sdl_context.event_pump().unwrap())
    }
}


impl PictureCreator {
    pub fn create_picture<'a>(&'a self) -> Picture<'a> {
        Picture {
            texture : self.texture_creator
                          .create_texture_streaming(FORMAT, WIDTH as u32,
                                                    HEIGHT as u32)
                          .unwrap(),
        }
    }
}

impl<'a> Picture<'a> {
    pub fn update(&mut self, pixeldata : &[u8]) {
        self.texture.update(None, &pixeldata, WIDTH*BYTES_PER_PIXEL).unwrap();
    }
}
