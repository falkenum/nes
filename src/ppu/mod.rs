
use super::graphics::SCREEN_SIZE;
use super::{ ComponentRc, Memory, Cartridge };

#[cfg(test)]
mod tests;

pub struct PPU {
    mem      : PPUMem,
    pixeldata : [u8; SCREEN_SIZE],

    control  : u8,
    mask     : u8,
    oam_addr : u8,
    scroll   : u8,

    address  : u16,
    address_first_write : bool,
    address_first_val : u8,

    // required to correctly emulate reads from $2007,
    // look at https://wiki.nesdev.com/w/index.php/PPU_registers,
    // in the section about PPUDATA read buffer
    data_readbuf : u8,
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
            PALETTE_RAM_FIRST...PALETTE_RAM_LAST => {
                let mut addr = addr % PALETTE_RAM_SIZE;

                // the palette addrs ending in 0x0, 0x4, 0x8, 0xC should mirror down
                if addr & 0b11 == 0 {
                    addr &= 0b1100;
                }

                self.palette_ram[addr as usize]
            },
            _ => panic!("invalid ppu address"),
        }
    }
    fn storeb(&mut self, addr : u16, val : u8) {
        match addr {
            CART_FIRST...CART_LAST => self.cart.borrow_mut().storeb(addr, val),
            PALETTE_RAM_FIRST...PALETTE_RAM_LAST => {
                // self.palette_ram[(addr % PALETTE_RAM_SIZE) as usize] = val,
                let mut addr = addr % PALETTE_RAM_SIZE;

                if addr & 0b11 == 0 {
                    addr &= 0b1100;
                }

                self.palette_ram[addr as usize] = val;
            }
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
            pixeldata : [0; SCREEN_SIZE],
            control  : 0,
            mask     : 0,
            oam_addr : 0,
            scroll   : 0,
            address  : 0,
            address_first_write  : true,
            address_first_val  : 0,
            data_readbuf : 0,
        }
    }

    pub fn reg_read(&mut self, reg_num : u8) -> u8 {
        use self::reg_id::*;
        match reg_num {

            CONTROL => unimplemented!(),
            MASK    => unimplemented!(),
            STATUS  => unimplemented!(),
            OAMADDR => unimplemented!(),
            OAMDATA => unimplemented!(),
            SCROLL  => unimplemented!(),
            ADDRESS => unimplemented!(),
            DATA    => {
                let addr = self.address;
                self.address += if self.control >> 2 == 1 { 0x20 } else { 0x01 };

                if addr < PALETTE_RAM_FIRST {
                    let ret = self.data_readbuf;
                    self.data_readbuf = self.mem.loadb(addr);
                    ret
                }
                // if it's in palette ram, don't use the read buffer
                else {
                    self.mem.loadb(addr)
                }
            },
            _ => panic!("invalid ppu reg num"),
        }
    }

    pub fn reg_write(&mut self, reg_num : u8, val : u8) {
        use self::reg_id::*;
        match reg_num {
            CONTROL => self.control = val,
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
                self.address += if self.control >> 2 == 1 { 0x20 } else { 0x01 };
            },
            _ => panic!("invalid ppu reg num"),
        }
    }

    fn render_scanline(&mut self, scanline : u16) {
        // TODO palette 0 always goes to 0x3F00
        // palette mirrors
        // control:
        // base nt
        // vram increment
        // sprite pt
        // bg pt
        // ext stuff (ignore)
        // generate nmi

        let nt_base_bits = (self.control & 0b00000011) as u16;
        let nt_base = 0x2000 | (nt_base_bits << 10);

        let nt_row = (scanline & 0b11111_000) >> 3;
        let tile_row = scanline & 0b00000_111;

        debug_assert!(nt_base == 0x2000 ||
                      nt_base == 0x2400 ||
                      nt_base == 0x2800 ||
                      nt_base == 0x2C00, "invalid nt_base: 0x{:x}", nt_base);

        for nt_col in 0..32 {
            let nt_addr = nt_base + nt_col + 32*nt_row;
            let tile_num = self.mem.loadb(nt_addr);
            let tile_addr = (tile_num as u16) << 4;

            let pattern_low  = self.mem.loadb(tile_addr + tile_row);
            let pattern_high = self.mem.loadb(tile_addr + tile_row + 8);

            // loop through pairs of numbers like (0, 7), (1, 6), (2, 5), etc
            for (tile_col, shamt) in (0..8).rev().enumerate() {

                // low two bits of palette index, from pattern table
                let palette_low = ((pattern_low  >> shamt) & 0b1) |
                                (((pattern_high >> shamt) & 0b1) << 1);

                let palette_high = {

                    // 10 bit nametable index (1024 bytes per nt)
                    let nt_index = nt_addr & 0x3FF;
                    let at_index = ((nt_index & 0b11100_00000) >> 4) |
                                   ((nt_index & 0b00000_11100) >> 2);

                    let at_base = nt_base + 0x3C0;

                    let at_val = self.mem.loadb(at_base + at_index);

                    let tile_attr_quadrant = ((nt_index & 0b00010_00000) >> 5) |
                                             ((nt_index & 0b00000_00010) >> 1);

                    match tile_attr_quadrant {
                        // top left
                        0 => (at_val & 0b00000011) << 2,
                        // top right
                        1 => (at_val & 0b00001100) << 0,
                        // bottom left
                        2 => (at_val & 0b00110000) >> 2,
                        // bottom right
                        3 => (at_val & 0b11000000) >> 4,
                        _ => panic!("messed up somewhere, quadrant: {}",
                                     tile_attr_quadrant)
                    }
                };


                // 0's in a pattern always refers to universal background 0x3F00
                let palette_i = palette_low |
                    if palette_low == 0 {0} else {palette_high};

                debug_assert!(palette_i < 16,
                            "invalid palette_i {:b}", palette_i);

                let color =
                    self.mem.loadb(0x3F00 + palette_i as u16) as usize;

                debug_assert!(color < 64, "invalid color {:x}", color);

                let color_bgr = PALETTE_BGR[color];

                let tile_row = tile_row as usize;
                let tile_col = tile_col as usize;
                let nt_row = nt_row as usize;
                let nt_col = nt_col as usize;

                let x = nt_row*256*3*8 + nt_col*8*3 +
                        tile_row*256*3 + tile_col*3;

                self.pixeldata[x + 0] = color_bgr.0;
                self.pixeldata[x + 1] = color_bgr.1;
                self.pixeldata[x + 2] = color_bgr.2;
            }
        }

    }

    pub fn render(&mut self, picture : &mut super::graphics::Picture) {
        for i in 0..240 {
            self.render_scanline(i);
        }

        picture.update(&self.pixeldata);
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
    (252, 252, 000), (248, 216, 248), (000, 000, 000), (000, 000, 000),
];
