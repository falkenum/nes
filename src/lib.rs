#![allow(dead_code)]

extern crate sdl2;

mod cartridge;
mod cpu;
mod graphics;

mod ppu {
    use super::{ Cartridge, RefCell, Rc };

    pub struct PPU {
        vram : [u8; 100],
        cart : Rc<Cartridge>,
    }

    impl PPU {
        pub fn new(_cart : Rc<Cartridge>) -> PPU {
            PPU {
                vram : [255; 100],
                cart : _cart,
            }
        }
        pub fn render(&self, picture : &mut super::graphics::Picture) {
            picture.update(&self.vram);
        }
    }
}

mod apu {
    pub struct APU {}
    impl APU {
        pub fn new() -> APU {
            APU {}
        }
    }
}

use cartridge::Cartridge;
use cpu::CPU;
use ppu::PPU;
use apu::APU;
use graphics::Screen;
use std::cell::RefCell;
use std::rc::Rc;

pub struct NES {
    cpu : CPU,
    ppu : PPU,
    apu : APU,
    screen : Screen,
}

impl NES {

    pub fn new(cart : Cartridge) -> NES {
        let x = Rc::new(cart);
        NES {
            cpu : CPU::new(Rc::clone(&x)),
            ppu : PPU::new(Rc::clone(&x)),
            apu : APU::new(),
            screen : Screen::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), &str> {

        let picture_creator = self.screen.picture_creator();
        let mut picture = picture_creator.create_picture();

        self.ppu.render(&mut picture);
        self.screen.update_and_show(&picture);

        Ok(())
    }
}

trait Memory {
    fn storeb(&mut self, addr : u16, val : u8);
}
