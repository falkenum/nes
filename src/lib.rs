#![allow(dead_code)]

extern crate sdl2;

pub mod cartridge;
mod cpu;
mod graphics;
mod ppu;
mod apu;
mod input;
mod controller;

use cartridge::Cartridge;
use controller::Controller;
use cpu::CPU;
use ppu::PPU;
use apu::APU;
use graphics::Screen;
use input::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn run_emulator(cart : Cartridge) {
    let mut screen = Screen::new();

    let cart  = Component::new(cart);
    let ppu   = Component::new(PPU::new(cart.new_ref()));
    let apu   = Component::new(APU::new());
    let input = Component::new(screen.emulator_input());
    let controller = Component::new(Controller::new());

    let cpu = CPU::new(
        cart.new_ref(), ppu.new_ref(), apu.new_ref(), controller.new_ref());

    // current version of rust-sdl2 requires that I use a
    // TextureCreator that is alive as long as the Texture is,
    // which is the reason I ended up with this annoying
    // picture_creator thing
    let picture_creator = screen.picture_creator();
    let mut picture = picture_creator.create_picture();

    ppu.borrow().render(&mut picture);
    screen.update_and_show(&picture);

    'running: loop {
        for event in input.borrow_mut().events() {
            match event {
                EmulatorEvent::Exit => break 'running,
                EmulatorEvent::Continue => (),
                EmulatorEvent::ControllerEvent { status, button } =>
                    controller.borrow_mut().update(status, button),
            }
        }
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }

}

trait Memory {
    fn storeb(&mut self, addr : u16, val : u8);
    fn loadb(&self, addr : u16) -> u8;
}

// I am using Rc/RefCell because both the cpu and ppu
// must be able to access the Cartridge (shared ownership)
pub struct Component<T> (
    Rc<RefCell<T>>
);

impl<T> Component<T> {
    fn new(val : T) -> Component<T> {
        Component (
            Rc::new(RefCell::new(val))
        )
    }

    fn new_ref(&self) -> Component<T> {
        Component (
            Rc::clone(&self.0)
        )
    }
}

impl<T> std::ops::Deref for Component<T> {
    type Target = Rc<RefCell<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
