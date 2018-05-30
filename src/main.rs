extern crate nes;
// use nes::cartridge::Cartridge;
// use nes::cpu::CPU;

fn main() {

    let args : Vec<String> = std::env::args().collect();
    let filename = if args.len() == 2 {
        args[1].clone()
    }
    else {
        String::from("tests/test.nes")
    };

    nes::run_emulator(nes::cartridge::Cartridge::from_ines_file(&filename));
}
