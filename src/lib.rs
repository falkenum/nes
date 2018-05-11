#![allow(dead_code)]

extern crate sdl2;

pub struct NES {
    cart : Option<cartridge::Cartridge>,
    cpu : cpu::CPU,
    ppu : ppu::PPU,
    apu : apu::APU,
    first_controller : controller::Controller,
    second_controller : controller::Controller,
}

impl NES {
    fn run(&mut self) {

        let mut s = screen::Screen::new();
        let mut p = s.new_picture();

        self.ppu.render(&mut p);
    }
}

mod cartridge;
mod cpu;
mod screen;

mod ppu {
    pub struct PPU {
        vram : [u8; 100],
    }
    impl PPU {

        pub fn render(&self, picture : &mut super::screen::Picture) {
            picture.update(&self.vram);
        }
    }
}
mod apu {
    pub struct APU {}
}
mod controller {
    pub struct Controller {}
}
