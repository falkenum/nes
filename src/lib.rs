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
pub mod cpu;

pub mod ppu {
    pub struct PPU {}
}
pub mod apu {
    pub struct APU {}
}
pub mod controller {
    pub struct Controller {}
}
