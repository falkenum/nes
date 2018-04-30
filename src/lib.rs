#![allow(dead_code)]

pub struct NES {
    cart : Option<cartridge::Cartridge>,
    cpu : cpu::CPU,
    ppu : ppu::PPU,
    apu : apu::APU,
    first_controller : controller::Controller,
    second_controller : controller::Controller,
}

pub mod cartridge;
mod cpu;

mod ppu {
    pub struct PPU {}
}
mod apu {
    pub struct APU {}
}
mod controller {
    pub struct Controller {}
}
