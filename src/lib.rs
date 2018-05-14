#![allow(dead_code)]

extern crate sdl2;

pub mod cartridge;
mod cpu;
mod graphics;
mod ppu;
mod apu;
use sdl2::event::Event;

enum EmulatorEvent {
    Controller,
    Continue,
    Exit,
}

struct EmulatorInput {
    pump : sdl2::EventPump,
}

impl EmulatorInput {
    fn input_events(&mut self) -> Vec<EmulatorEvent> {


        self.pump.poll_iter().map( |event|
            match event {
                Event::Quit {..} |
                Event::KeyDown {..} => EmulatorEvent::Exit,
                _ => EmulatorEvent::Continue,
            }
        ).collect()
    }
}



use cartridge::Cartridge;
use cpu::CPU;
use ppu::PPU;
use apu::APU;
use graphics::Screen;
use std::cell::RefCell;
use std::rc::Rc;

pub fn run_emulator(cart : Cartridge) {
    // I am using Rc/RefCell because both the cpu and ppu
    // must be able to access the Cartridge (shared ownership)
    let cart_ref = Rc::new(RefCell::new(cart));

    let mut screen = Screen::new();
    let input = screen.emulator_input();
    let mut cpu = CPU::new(Rc::clone(&cart_ref));
    let mut ppu = PPU::new(Rc::clone(&cart_ref));
    let _apu = APU::new();

    // current version of rust-sdl2 requires that I use a
    // TextureCreator that is alive as long as the Texture is,
    // which is the reason I ended up with this annoying
    // picture_creator thing
    let picture_creator = screen.picture_creator();
    let mut picture = picture_creator.create_picture();

    ppu.render(&mut picture);
    screen.update_and_show(&picture);

    loop {
        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }

}

trait Memory {
    fn storeb(&mut self, addr : u16, val : u8);
    fn loadb(&self, addr : u16) -> u8;
}
