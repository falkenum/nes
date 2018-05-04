extern crate nes;
use nes::cartridge::Cartridge;
use nes::cpu::CPU;

/// document main
fn main() {
    // let c = Cartridge::from_ines_file(String::from("roms/mario.nes"));
    let mut c = CPU::new();
    c.load_cartridge(Cartridge::from_ines_file(String::from("roms/mario.nes")));

    c.step();
    c.step();
    c.step();
    c.step();
    c.step();
    c.step();
}
