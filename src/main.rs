extern crate nes;
// use nes::cartridge::Cartridge;
// use nes::cpu::CPU;

fn main() {
    let args : Vec<String> = std::env::args().collect();

    match args.len() {
        1 => panic!("need rom filename"),
        2 => (),
        // 3 =>
        //     if args[2] == String::from("-d") {
        //         debug_mode = true
        //     } else {
        //         panic!("invalid argument {}", args[2])
        //     },
        _ => panic!("too many arguments"),
    }

    let filename = args[1].clone();

    nes::run_emulator(nes::cartridge::Cartridge::from_ines_file(&filename));
}
