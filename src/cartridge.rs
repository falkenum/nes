
pub struct Cartridge {
    prgrom : Vec<u8>,
    chrrom : Vec<u8>,
}

impl Cartridge {
    // used for debugging/testing in places where a Cartridge
    // placeholder is needed
    pub fn test() -> Cartridge {
        let mut new_prgrom = Vec::new();
        new_prgrom.resize(0x8000, 0);
        Cartridge {
            prgrom : new_prgrom,
            chrrom : Vec::new(),
        }
    }

    // https://wiki.nesdev.com/w/index.php/INES
    pub fn from_ines_file(filename : String) -> Cartridge {
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

        println!("loaded cartridge");
        println!("num prgrom banks: {}; total prgrom size: {}",
            num_prgrom_banks, prgrom_size);
        println!("num chrrom banks: {}; total chrrom size: {}",
            num_chrrom_banks, chrrom_size);

        Cartridge { prgrom : new_prgrom, chrrom : new_chrrom }
    }
}

const ROM_FIRST : usize = 0x8000;
const ROM_LAST : usize = 0xFFFF;

// TODO right now I'm assuming it's NROM256
use std::ops::{ Index, IndexMut };
impl Index<usize> for Cartridge {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            ROM_FIRST...ROM_LAST => &self.prgrom[index - ROM_FIRST],
            _ => panic!("invalid cartridge address"),
        }
    }
}

impl IndexMut<usize> for Cartridge {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            ROM_FIRST...ROM_LAST => &mut self.prgrom[index - ROM_FIRST],
            _ => panic!("can't modify cartridge rom"),
        }
    }
}
