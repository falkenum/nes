use ::sdl2::{ render, pixels, video };
use self::pixels::PixelFormatEnum;

const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT : u32 = 800;
const FORMAT : PixelFormatEnum = PixelFormatEnum::BGR24;
const WIDTH : usize = 256;
const HEIGHT : usize = 240;
const BYTES_PER_PIXEL : usize = 3;
pub const SCREEN_SIZE : usize = WIDTH * HEIGHT * BYTES_PER_PIXEL;


pub struct EmulatorGraphics {
    canvas : render::Canvas<video::Window>,
    texture_creator : render::TextureCreator<video::WindowContext>,
}

impl EmulatorGraphics {
    pub fn update(&mut self, pixeldata : &[u8]) {
        let mut texture = self.texture_creator
                          .create_texture_streaming(FORMAT, WIDTH as u32,
                                                    HEIGHT as u32)
                          .unwrap();
        texture.update(None, pixeldata, WIDTH*BYTES_PER_PIXEL).unwrap();

        self.canvas.copy(&texture, None, None).unwrap();
        self.canvas.present();
    }

    pub fn new(sdl_context : &::Sdl) -> EmulatorGraphics {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("NES", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();

        EmulatorGraphics {
            canvas : canvas,
            texture_creator : texture_creator,
        }
    }
}
