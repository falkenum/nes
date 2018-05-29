
use super::graphics::SCREEN_SIZE;
use super::{ ComponentRc, Memory, Cartridge };

const NUM_REGS : usize = 8;
pub struct PPU {
    mem      : PPUMem,
    control  : u8,
    mask     : u8,
    oam_addr : u8,
    oam_data : u8,
    scroll   : u8,
    address  : u8,
    data     : u8,
}

// enum MirroringType {
//     Vertical,
//     Horizontal,
// }

const PALETTE_RAM_SIZE  : u16 = 0x0020;
const PALETTE_RAM_FIRST : u16 = 0x3F00;
const PALETTE_RAM_LAST  : u16 = 0x3FFF;
const CART_FIRST        : u16 = 0x0000;
// including vram
const CART_LAST         : u16 = 0x3EFF;

const CONTROL : u8 = 0;
const MASK    : u8 = 1;
const STATUS  : u8 = 2;
const OAMADDR : u8 = 3;
const OAMDATA : u8 = 4;
const SCROLL  : u8 = 5;
const ADDRESS : u8 = 6;
const DATA    : u8 = 7;

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
            oam_data : 0,
            scroll   : 0,
            address  : 0,
            data     : 0,
        }
    }

    pub fn reg_read(&self, reg_num : u8) -> u8 {
        match reg_num {

            CONTROL => 0,
            MASK    => 0,
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

    pub fn render(&self, picture : &mut super::graphics::Picture) {

        // // each tile is 16 bytes
        // let tile_addr = (tile_num as u16) << 4;


        let mut pixeldata : [u8; SCREEN_SIZE] = fill_color(0, 0, 255);
        pixeldata[1] = 255;
        pixeldata[2] = 255;

        const BASE : u16 = 0x0000;
        for i in 0..8  {
            let pattern_low  = self.mem.loadb(BASE + i);
            let pattern_high = self.mem.loadb(BASE + i + 8);

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

                let color_bgr = &PALETTE_BGR[(color*3)..(color*3 + 3)];

                let j = j as u16;
                let x = (i*256*3 + j*3) as usize;

                pixeldata[x + 0] = color_bgr[0];
                pixeldata[x + 1] = color_bgr[1];
                pixeldata[x + 2] = color_bgr[2];
            }
        }

        // for each tile in the nametable, get the pattern and the palette num
        // generate that tile

        // let mut name_i = 0;
        // let mut attr_i = 0;

        // for _attr_row in 0..8 {
        //     for _attr_col in 0..8 {
        //         let val = self.mem.loadb(0x23C0 + attr_i);

        //         let topleft  = (val & 0b00000011) << 2;
        //         let topright =  val & 0b00001100;
        //         let botleft  = (val & 0b00110000) >> 2;
        //         let botright = (val & 0b11000000) >> 4;

        //         let i = name_i;

        //         let tile = self.mem.loadb(i);


        //         name_i += 4;
        //         attr_i += 1;
        //     }
        //     name_i += 0x60;


        //     let topright = val & 0b00001100;
        //     let botleft  = val & 0b00110000;
        //     let botright = val & 0b11000000;


        // }

        // TODO remove magic numbers, do all nametables
        // for nametable_addr in 0x2000..0x23C0 {
        //     // get the value from the nametable at this index
        //     let tile_num = self.mem.loadb(nametable_addr);

        //     // e.g. if tile_num is 5, then its base addr in pattern table is 0x50
        //     // (16 bytes per tile)
        //     let tile_addr = (tile_num as u16) << 4;

        //     let nametable_i = nametable_addr - 0x2000;
        //     let attr_i = (nametable_i & 0b11100_00000) >> 4 |
        //                  (nametable_i & 0b00000_11100) >> 2;

        //     let attr_section = (nametable_i & 0b00010_00000) >> 5 |
        //                        (nametable_i & 0b00000_00010) >> 1;

        //     let val = self.mem.loadb(0x23C0 + attr_i);

        //     let topleft  = (val & 0b00000011) << 2;
        //     let topright =  val & 0b00001100;
        //     let botleft  = (val & 0b00110000) >> 2;
        //     let botright = (val & 0b11000000) >> 4;

        //     // high 2 bits of palette index
        //     let palette_high = match attr_section {
        //         0b00 => topleft,
        //         0b01 => topright,
        //         0b10 => botleft,
        //         0b11 => botright,
        //         _    => panic!(),
        //     };

        //     for addr in tile_addr..(tile_addr + 8) {
        //         let low_bits = self.mem.loadb(addr);
        //         let high_bits = self.mem.loadb(addr + 8);

        //         for j in (0..8).rev() {

        //             // low two bits of palette index, from pattern table
        //             let palette_low = (low_bits >> j) | ((high_bits >> j) << 1);

        //             let palette_i = palette_high | palette_low;

        //             assert!(palette_i < 16);

        //             let color = self.mem.loadb(0x3F00 + palette_i as u16) as usize;
        //             let color = &PALETTE_BGR[color..(color+3)];

        //             // get attr table byte
        //             //    get high 2 bits of palette_i for 4x4=16 tiles
        //             //    for each of 16 tiles, get low two bits from pt, get color
        //             //      from palette, set pixel data

        //         }
        //     }
        // }

        picture.update(&pixeldata);
    }
}

#[cfg(test)]
mod tests {
    use super::{ PPU, Memory, Cartridge };
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
    fn rendering() {

    }
}

static PALETTE_BGR: [u8; 192] = [
    124, 124, 124, 252, 0,   0,   188, 0,   0,   188, 40,  68,
    132, 0,   148, 32,  0,   168, 0,   16,  168, 0,   20,  136,
    0,   48,  80,  0,   120, 0,   0,   104, 0,   0,   88,  0,
    88,  64,  0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    188, 188, 188, 248, 120, 0,   248, 88,  0,   252, 68,  104,
    204, 0,   216, 88,  0,   228, 0,   56,  248, 16,  92,  228,
    0,   124, 172, 0,   184, 0,   0,   168, 0,   68,  168, 0,
    136, 136, 0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    248, 248, 248, 252, 188, 60,  252, 136, 104, 248, 120, 152,
    248, 120, 248, 152, 88,  248, 88,  120, 248, 68,  160, 252,
    0,   184, 248, 24,  248, 184, 84,  216, 88,  152, 248, 88,
    216, 232, 0,   120, 120, 120, 0,   0,   0,   0,   0,   0,
    252, 252, 252, 252, 228, 164, 248, 184, 184, 248, 184, 216,
    248, 184, 248, 192, 164, 248, 176, 208, 240, 168, 224, 252,
    120, 216, 248, 120, 248, 216, 184, 248, 184, 216, 248, 184,
    252, 252, 0,   248, 216, 248, 0,   0,   0,   0,   0,   0
];
