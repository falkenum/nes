extern crate nes;
use nes::cartridge::Cartridge;
use nes::cpu::CPU;

fn main() {
    // let c = Cartridge::from_ines_file(String::from("roms/mario.nes"));
    let mut c = CPU::new();
    c.load_cartridge(Cartridge::from_ines_file(String::from("nestest.nes")));

    while c.get_pc() != 0 {
        c.step();
    }
    println!("{:?}", c);
}
