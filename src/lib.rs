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
    let mut input = screen.emulator_input();

    let cart  = ComponentRc::new(cart);
    let ppu   = ComponentRc::new(PPU::new(cart.new_ref()));
    let apu   = ComponentRc::new(APU::new());
    let controller = ComponentRc::new(Controller::new());

    let mut cpu = CPU::new(
        cart.new_ref(), ppu.new_ref(), apu.new_ref(), controller.new_ref());

    // current version of rust-sdl2 requires that I use a
    // TextureCreator that is alive as long as the Texture is,
    // which is the reason I ended up with this annoying
    // picture_creator thing
    let picture_creator = screen.picture_creator();
    let mut picture = picture_creator.create_picture();


    use std::time::SystemTime;
    let start = SystemTime::now();
    let mut cpu_cycles : usize = 0;
    let mut num_frames : usize = 0;

    cpu.send_reset();

    'running: loop {

        // cpu during rendering
        while cpu_cycles < 114*241 {
            cpu_cycles += cpu.step() as usize;
        }
        cpu_cycles = 0;

        // start vblank
        ppu.borrow_mut().set_vblank();

        if ppu.borrow().nmi_enabled() {
            cpu.send_nmi();
        }

        // cpu during vblank
        while cpu_cycles < 114*20 {
            cpu_cycles += cpu.step() as usize;
        }
        cpu_cycles = 0;

        // ppu rendering
        ppu.borrow_mut().clear_vblank();
        ppu.borrow_mut().render(&mut picture);
        screen.update_and_show(&picture);

        for event in input.events() {
            match event {
                EmulatorEvent::Exit => break 'running,
                EmulatorEvent::Continue => (),
                EmulatorEvent::ControllerEvent { action, button } =>
                    controller.borrow_mut().update(action, button),
            }
        }
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
        num_frames += 1;
    }

    let duration = start.elapsed().unwrap();

    let freq = num_frames as f64 /
        (duration.as_secs() as f64 +
         (duration.subsec_nanos() as f64) / 1_000_000_000f64);

    println!("{:.2} frames/sec", freq);
}

trait Memory {
    fn storeb(&mut self, addr : u16, val : u8);
    fn loadb(&self, addr : u16) -> u8;
}

pub struct ComponentRc<T> (
    Rc<RefCell<T>>
);

impl<T> ComponentRc<T> {
    fn new(val : T) -> ComponentRc<T> {
        ComponentRc (
            Rc::new(RefCell::new(val))
        )
    }

    fn new_ref(&self) -> ComponentRc<T> {
        ComponentRc (
            Rc::clone(&self.0)
        )
    }
}

impl<T> std::ops::Deref for ComponentRc<T> {
    type Target = Rc<RefCell<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
