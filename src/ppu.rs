
use super::graphics::SCREEN_SIZE;
use super::{ ComponentRc, Memory, Cartridge };

pub struct PPU {
    mem      : PPUMem,
    control  : u8,
    mask     : u8,
    oam_addr : u8,
    // oam_data : u8,
    scroll   : u8,
    address  : u16,
    address_first_write : bool,
    address_first_val : u8,
    // data     : u8,
}

const PALETTE_RAM_SIZE  : u16 = 0x0020;
const PALETTE_RAM_FIRST : u16 = 0x3F00;
const PALETTE_RAM_LAST  : u16 = 0x3FFF;
const CART_FIRST        : u16 = 0x0000;
const CART_LAST         : u16 = 0x3EFF;

mod reg_id {
    pub const CONTROL : u8 = 0;
    pub const MASK    : u8 = 1;
    pub const STATUS  : u8 = 2;
    pub const OAMADDR : u8 = 3;
    pub const OAMDATA : u8 = 4;
    pub const SCROLL  : u8 = 5;
    pub const ADDRESS : u8 = 6;
    pub const DATA    : u8 = 7;
}

struct PPUMem {
    // 0000 - 0FFF : Pattern table 0
    // 1000 - 1FFF : Pattern table 1
    // 2000 - 23FF : Nametable 0
    // 2400 - 27FF : Nametable 1
    // 2800 - 2BFF : Nametable 2
    // 2C00 - 2FFF : Nametable 3
    // 3000 - 3EFF : Mirror of 2000 - 2EFF
    // 3F00 - 3F1F : Palette RAM
    // 3F20 - 3FFF : mirrors of 3F20 - 3FFF
    cart : ComponentRc<Cartridge>,
    palette_ram : [u8; PALETTE_RAM_SIZE as usize],
}

// TODO right now only vertical mirroring is suppported
// need to update the impl Memory to index into vram appropriately for
// horizontal mirroring.
impl Memory for PPUMem {
    fn loadb(&self, addr : u16) -> u8 {
        match addr {
            CART_FIRST...CART_LAST => self.cart.borrow().loadb(addr),
            PALETTE_RAM_FIRST...PALETTE_RAM_LAST =>
                self.palette_ram[(addr % PALETTE_RAM_SIZE) as usize],
            _ => panic!("invalid ppu address"),
        }
    }
    fn storeb(&mut self, addr : u16, val : u8) {
        match addr {
            CART_FIRST...CART_LAST => self.cart.borrow_mut().storeb(addr, val),
            PALETTE_RAM_FIRST...PALETTE_RAM_LAST =>
                self.palette_ram[(addr % PALETTE_RAM_SIZE) as usize] = val,
            _ => panic!("invalid ppu address"),
        }
    }
}

// helper/utility
fn fill_color(r : u8, g : u8, b : u8) -> [u8; SCREEN_SIZE] {
    // BGR24
    let mut data = [0; SCREEN_SIZE];

    {
        let mut i = data.iter_mut();
        for _ in 0..(SCREEN_SIZE / 3) {
            *i.next().unwrap() = b;
            *i.next().unwrap() = g;
            *i.next().unwrap() = r;
        }
    }

    data
}

impl PPU {
    pub fn new(cart : ComponentRc<Cartridge>) -> PPU {
        PPU {
            mem : PPUMem {
                cart : cart,
                palette_ram : [0; PALETTE_RAM_SIZE as usize],
            },
            control  : 0,
            mask     : 0,
            oam_addr : 0,
            // oam_data : 0,
            scroll   : 0,
            address  : 0,
            address_first_write  : true,
            address_first_val  : 0,
            // data     : 0,
        }
    }

    pub fn reg_read(&self, reg_num : u8) -> u8 {
        use self::reg_id::*;
        match reg_num {

            CONTROL => unimplemented!(),
            MASK    => unimplemented!(),
            STATUS  => unimplemented!(),
            OAMADDR => unimplemented!(),
            OAMDATA => unimplemented!(),
            SCROLL  => unimplemented!(),
            ADDRESS => unimplemented!(),
            DATA    => unimplemented!(),
            _ => panic!("invalid ppu reg num"),
        }
    }

    pub fn reg_write(&mut self, reg_num : u8, val : u8) {
        use self::reg_id::*;
        match reg_num {
            CONTROL => unimplemented!(),
            MASK    => unimplemented!(),
            STATUS  => unimplemented!(),
            OAMADDR => unimplemented!(),
            OAMDATA => unimplemented!(),
            SCROLL  => unimplemented!(),
            ADDRESS => {
                if self.address_first_write {
                    self.address_first_val = val;
                    self.address_first_write = false;
                }
                else {
                    // shift up the value from the first write
                    self.address = ((self.address_first_val as u16) << 8
                                    | val as u16) // add in the new value
                                    & 0x3FFF;     // mirror those above 0x3FFF
                    self.address_first_write = true;
                }
            },
            DATA    => {
                self.mem.storeb(self.address, val);
                self.address += 1;
            },
            _ => panic!("invalid ppu reg num"),
        }
    }

    pub fn render(&self, picture : &mut super::graphics::Picture) {

        let mut pixeldata : [u8; SCREEN_SIZE] = fill_color(0, 0, 252);

        // println!("palette ");
        // println!("0x3F00: {:02x}", self.mem.loadb(0x3F00));
        // println!("0x3F01: {:02x}", self.mem.loadb(0x3F01));
        // println!("0x3F02: {:02x}", self.mem.loadb(0x3F02));
        // println!("0x3F03: {:02x}", self.mem.loadb(0x3F03));

        const BASE : u16 = 0x0000;
        for i in 0..8 {
            let pattern_low  = self.mem.loadb(BASE + i);
            let pattern_high = self.mem.loadb(BASE + i + 8);


            // loop through pairs of numbers like (0, 7), (1, 6), (2, 5), etc
            for (j, k) in (0..8).rev().enumerate() {

                // low two bits of palette index, from pattern table
                let palette_low = ((pattern_low  >> k) & 0b1) |
                                 (((pattern_high >> k) & 0b1) << 1);

                // from attr table
                let palette_high = (self.mem.loadb(0x23C0) & 0b00000011) << 2;


                let palette_i = palette_high | palette_low;

                // println!("high: {:b}, low: {:b}", palette_high, palette_low);

                if palette_i >= 16 {
                    panic!("invalid palette_i {:b}", palette_i);

                }
                // assert!(palette_i < 16);

                let color = self.mem.loadb(0x3F00 + palette_i as u16) as usize;
                assert!(color < 64);

                let color_bgr = PALETTE_BGR[color];

                let j = j as u16;
                let x = (i*256*3 + j*3) as usize;

                // print!("({:x};{:02x}: {:03}, {:03}, {:03}) |",
                //        palette_i, color, color_bgr.0, color_bgr.1, color_bgr.2);

                pixeldata[x + 0] = color_bgr.0;
                pixeldata[x + 1] = color_bgr.1;
                pixeldata[x + 2] = color_bgr.2;
            }

            // println!("");
        }

        picture.update(&pixeldata);
    }
}

#[cfg(test)]
mod tests {
    use super::{ PPU, Memory, Cartridge };
    use super::reg_id::*;
    #[test]
    fn mappings() {
        let mut p = PPU::new(Cartridge::test_ref());
        p.mem.storeb(0x2000, 5);
        assert_eq!(p.mem.loadb(0x3000), 5);
        p.mem.storeb(0x2EFF, 5);
        assert_eq!(p.mem.loadb(0x3EFF), 5);
        p.mem.storeb(0x3F00, 5);
        assert_eq!(p.mem.loadb(0x3F20), 5);
        p.mem.storeb(0x3F1F, 5);
        assert_eq!(p.mem.loadb(0x3FFF), 5);
    }

    #[test]
    fn address_reg() {
        let mut p = PPU::new(Cartridge::test_ref());
        assert_eq!(p.address, 0x0000);

        p.reg_write(ADDRESS, 0x3F);
        assert_eq!(p.address, 0x0000);
        p.reg_write(ADDRESS, 0x00);
        assert_eq!(p.address, 0x3F00);

        p.reg_write(ADDRESS, 0x12);
        assert_eq!(p.address, 0x3F00);
        p.reg_write(ADDRESS, 0x34);
        assert_eq!(p.address, 0x1234);

        p.reg_write(ADDRESS, 0x55);
        assert_eq!(p.address, 0x1234);
        p.reg_write(ADDRESS, 0x55);
        assert_eq!(p.address, 0x1555);
    }

    #[test]
    fn data_reg() {
        let mut p = PPU::new(Cartridge::test_ref());
        assert_eq!(p.address, 0x0000);

        p.reg_write(ADDRESS, 0x3F);
        assert_eq!(p.address, 0x0000);
        p.reg_write(ADDRESS, 0x00);
        assert_eq!(p.address, 0x3F00);

        p.reg_write(DATA, 0xFF);

        assert_eq!(p.mem.loadb(0x3F00), 0xFF);
    }

}

static PALETTE_BGR: [(u8, u8, u8); 64] = [
    (124, 124, 124), (252, 000, 000), (188, 000, 000), (188, 040, 068),
    (132, 000, 148), (032, 000, 168), (000, 016, 168), (000, 020, 136),
    (000, 048, 080), (000, 120, 000), (000, 104, 000), (000, 088, 000),
    (088, 064, 000), (000, 000, 000), (000, 000, 000), (000, 000, 000),
    (188, 188, 188), (248, 120, 000), (248, 088, 000), (252, 068, 104),
    (204, 000, 216), (088, 000, 228), (000, 056, 248), (016, 092, 228),
    (000, 124, 172), (000, 184, 000), (000, 168, 000), (068, 168, 000),
    (136, 136, 000), (000, 000, 000), (000, 000, 000), (000, 000, 000),
    (248, 248, 248), (252, 188, 060), (252, 136, 104), (248, 120, 152),
    (248, 120, 248), (152, 088, 248), (088, 120, 248), (068, 160, 252),
    (000, 184, 248), (024, 248, 184), (084, 216, 088), (152, 248, 088),
    (216, 232, 000), (120, 120, 120), (000, 000, 000), (000, 000, 000),
    (252, 252, 252), (252, 228, 164), (248, 184, 184), (248, 184, 216),
    (248, 184, 248), (192, 164, 248), (176, 208, 240), (168, 224, 252),
    (120, 216, 248), (120, 248, 216), (184, 248, 184), (216, 248, 184),
    (252, 252, 000), (248, 216, 248), (000, 000, 000), (000, 000, 000)
];
