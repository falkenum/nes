use super::{ Memory, ComponentRc };

pub struct Cartridge {
    prgrom_size : u16,
    prgrom : Vec<u8>,
    chrrom : Vec<u8>,
    vram : [u8; VRAM_SIZE as usize],
}

impl Cartridge {
    // used for debugging/testing in places where a Cartridge
    // placeholder is needed
    pub fn test_ref() -> ComponentRc<Cartridge> {
        let mut new_prgrom = Vec::new();
        new_prgrom.resize(0x8000, 0);

        let mut new_chrrom = Vec::new();
        new_chrrom.resize(0x2000, 0);
        ComponentRc::new(
            Cartridge {
                prgrom_size : new_prgrom.len() as u16,
                prgrom : new_prgrom,
                chrrom : new_chrrom,
                vram : [0; VRAM_SIZE as usize],
            }
        )
    }

    // https://wiki.nesdev.com/w/index.php/INES
    pub fn from_ines_file(filename : &str) -> Cartridge {
        use std::fs::File;
        use std::io::prelude::*;
        const HEADER_SIZE : usize = 16;
        const PRGROM_BANK_SIZE : usize = 16384;
        const CHRROM_BANK_SIZE : usize = 8192;

        let file = File::open(filename).expect("error opening file");
        let data : Vec<u8> = file.bytes()
                        .map(|r| r.expect("error reading data"))
                        .collect();

        let data = &data[..];

        let header = &data[..HEADER_SIZE];

        let num_prgrom_banks = header[4];
        let num_chrrom_banks = header[5];
        let flags6          = header[6];
        println!("flags 6: {:08b}", flags6);
        let flags7          = header[7];
        println!("flags 7: {:08b}", flags7);

        let prgrom_size = PRGROM_BANK_SIZE * num_prgrom_banks as usize;
        let chrrom_size = CHRROM_BANK_SIZE * num_chrrom_banks as usize;

        let prgrom_start = HEADER_SIZE;
        let prgrom_end   = prgrom_start + prgrom_size;

        let chrrom_start = prgrom_end;
        let chrrom_end   = chrrom_start + chrrom_size;

        let new_prgrom : Vec<u8> =
            data[prgrom_start..prgrom_end]
            .to_vec();
        let new_chrrom : Vec<u8> =
            data[chrrom_start..chrrom_end]
            .to_vec();

        // TODO check if there is still more data (invalid file)

        println!("loaded cartridge {}", filename);
        println!("num prgrom banks: {}; total prgrom size: {}k",
            num_prgrom_banks, prgrom_size / 1024);
        println!("num chrrom banks: {}; total chrrom size: {}k",
            num_chrrom_banks, chrrom_size / 1024);

        Cartridge {
            prgrom_size : new_prgrom.len() as u16,
            prgrom : new_prgrom,
            chrrom : new_chrrom,
            vram : [0; VRAM_SIZE as usize],
        }
    }
}

const CHR_FIRST : u16 = 0x0000;
const CHR_LAST : u16 = 0x1FFF;
const PRG_FIRST : u16 = 0x8000;
const PRG_LAST : u16 = 0xFFFF;

const VRAM_SIZE : u16 = 0x0800;
const VRAM_FIRST : u16 = 0x2000;
const VRAM_LAST : u16 = 0x3EFF;

/*
 I'm including VRAM in the cartridge because the cartridge can map ppu memory to
 VRAM or somewhere else. So it might not make sense from an OOP perspective,
 but it's easier to code.
*/

// TODO horizontal mirroring

// TODO right now I'm assuming it's NROM256
impl Memory for Cartridge {
    fn loadb(&self, addr : u16) -> u8 {
        match addr {
            CHR_FIRST...CHR_LAST => self.chrrom[addr as usize],
            VRAM_FIRST...VRAM_LAST => self.vram[(addr % VRAM_SIZE) as usize],
            PRG_FIRST...PRG_LAST => self.prgrom[((addr - PRG_FIRST) %
                                                 self.prgrom_size) as usize],
            _ => panic!("invalid cartridge address"),
        }
    }
    fn storeb(&mut self, addr : u16, val : u8) {
        match addr {
            CHR_FIRST...CHR_LAST => self.chrrom[addr as usize] = val,
            VRAM_FIRST...VRAM_LAST => self.vram[(addr % VRAM_SIZE) as usize] = val,
            PRG_FIRST...PRG_LAST => self.prgrom[((addr - PRG_FIRST) %
                                                self.prgrom_size) as usize] = val,
            _ => panic!("invalid cartridge address"),
        }
    }
}
