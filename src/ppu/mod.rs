
use super::graphics::SCREEN_SIZE;
use super::{ ComponentRc, Memory, Cartridge };

#[cfg(test)]
mod tests;

const OAM_SIZE : usize = 256;

// TODO byte 2 of sprites anded with E3
pub struct PPU {
    mem                 : PPUMem,
    oam                 : [u8; OAM_SIZE],
    pixeldata           : [u8; SCREEN_SIZE],

    control             : u8,
    mask                : u8,
    oam_addr            : u8,
    scroll              : u8,

    address             : u16,
    address_first_write : bool,
    address_first_val   : u8,

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

fn concat_palette_bits(low : u8, high : u8) -> u8 {
    // 0's in a pattern always refers to universal background 0x3F00
    low | if low == 0 {0} else {high}
}

enum SpriteRenderResult {
    Rendered,
    Overflowed,
}

impl PPU {
    pub fn new(cart : ComponentRc<Cartridge>) -> PPU {
        PPU {
            mem : PPUMem {
                cart : cart,
                palette_ram : [0; PALETTE_RAM_SIZE as usize],
            },
            pixeldata : [0; SCREEN_SIZE],
            oam       : [0xFF; OAM_SIZE], // init to FF so sprites are hidden
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

    pub fn test() -> PPU {
        PPU::new(Cartridge::test_ref())
    }

    pub fn reg_read(&mut self, reg_num : u8) -> u8 {
        use self::reg_id::*;
        match reg_num {

            CONTROL => 0,
            MASK    => 0,
            STATUS  => unimplemented!(),
            OAMADDR => 0,
            OAMDATA => self.oam[self.oam_addr as usize],
            SCROLL  => unimplemented!(),
            ADDRESS => 0,
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
            MASK    => self.mask = val,
            STATUS  => unimplemented!(),
            OAMADDR => self.oam_addr = val,
            OAMDATA => {
                self.oam[self.oam_addr as usize] = val;
                self.oam_addr = self.oam_addr.wrapping_add(1);
            },
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

    // fn render_tile_row(x : u8, y : u8, render_style : RenderStyle) {
    // }

    fn set_pixel(&mut self, x : u8, y : u8, color : usize) {
        let x = x as usize;
        let y = y as usize;

        let i = (y*256 + x) * 3;

        debug_assert!(color < 64, "invalid color {:x}", color);

        let color_bgr = PALETTE_BGR[color];

        self.pixeldata[i + 0] = color_bgr.0;
        self.pixeldata[i + 1] = color_bgr.1;
        self.pixeldata[i + 2] = color_bgr.2;
    }

    // returns high two bits for bg tile color
    fn get_bg_tile_attr(&self, nt_addr : u16) -> u8 {
        // 10 bit nametable index (1024 bytes per nt)
        let nt_index = nt_addr & 0x3FF;
        let at_index = ((nt_index & 0b11100_00000) >> 4) |
                        ((nt_index & 0b00000_11100) >> 2);

        let at_base = (nt_addr & 0xFC00) + 0x3C0;

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
    }

    fn render_scanline_bg(&mut self, scanline : u8) {

        // control:

        // sprite pt
        // generate nmi

        // TODO scrolling

        let nt_base_bits = (self.control & 0b00000011) as u16;
        let nt_base = 0x2000 | (nt_base_bits << 10);

        let nt_row = ((scanline & 0b11111_000) >> 3) as u8;
        let tile_row = (scanline & 0b00000_111) as u8;

        debug_assert!(nt_base == 0x2000 ||
                      nt_base == 0x2400 ||
                      nt_base == 0x2800 ||
                      nt_base == 0x2C00, "invalid nt_base: 0x{:x}", nt_base);

        for nt_col in 0..32 {
            let nt_addr = nt_base + nt_col as u16 + 32*(nt_row as u16);
            let tile_num = self.mem.loadb(nt_addr);

            // add offset if pt is at 0x1000
            let pt_base = (self.control as u16 & 0x10) << 8;

            let tile_addr = pt_base + ((tile_num as u16) << 4);

            let pattern_low  = self.mem.loadb(tile_addr + tile_row as u16);
            let pattern_high = self.mem.loadb(tile_addr + tile_row as u16 + 8);

            // loop through pairs of numbers like (0, 7), (1, 6), (2, 5), etc
            for (tile_col, shamt) in (0..8).rev().enumerate() {

                // can't figure out how to make it u8 by default...
                let tile_col = tile_col as u8;

                // low two bits of palette index, from pattern table
                let palette_low = ((pattern_low  >> shamt) & 0b1) |
                                 (((pattern_high >> shamt) & 0b1) << 1);

                let palette_high = self.get_bg_tile_attr(nt_addr);

                let palette_i = concat_palette_bits(palette_low, palette_high);

                let color = self.get_palette_color(palette_i);

                debug_assert!(nt_row < 30);
                debug_assert!(nt_col < 32);
                debug_assert!(tile_row < 8);
                debug_assert!(tile_col < 8);

                let x = nt_col*8 + tile_col;
                let y = nt_row*8 + tile_row;

                debug_assert!(y < 240);

                self.set_pixel(x, y, color);

            }
        }
    }

    fn get_palette_color(&self, palette_i : u8) -> usize {
        self.mem.loadb(0x3F00 + palette_i as u16) as usize
    }

    // TODO sprite zero hit
    // TODO sprite bg priority
    // TODO sprite overlap priority

    fn render_sprite(&mut self, sprite_num : u8, scanline : u8)
                     -> SpriteRenderResult {

        let sprite_i = (sprite_num*4) as usize;
        let y = self.oam[sprite_i+0] + 1;
        let x = self.oam[sprite_i+3];

        let attributes = self.oam[sprite_i+2];
        let vert_flip = (attributes & 0x80) == 0x80;
        let horiz_flip = (attributes & 0x40) == 0x40;
        let palette_high = 0x10 | ((attributes & 0x3) << 2);

        let tile_num = self.oam[sprite_i+1];

        let pt_base = (self.control as u16 & 0x08) << 9;
        debug_assert!(pt_base == 0x0000 || pt_base == 0x1000);

        let tile_addr = pt_base + ((tile_num as u16) << 4);

        let sprite_row = if vert_flip {
            7 - (scanline - y)
        }
        else {
            scanline - y
        };

        let pattern_low  = self.mem.loadb(tile_addr + sprite_row as u16);
        let pattern_high = self.mem.loadb(tile_addr + sprite_row as u16 + 8);

        let end_col = if x > 0xF8 {0xFF - x + 1} else {8};

        // if horiz flip, reverse pattern bits
        let shamt_list : Vec<(usize, u8)> = if horiz_flip {
            (0..end_col).enumerate().collect()
        }
        else {
            (0..end_col).rev().enumerate().collect()
        };


        for (sprite_col, shamt) in shamt_list {
            let sprite_col = sprite_col as u8;

            // low two bits of palette index, from pattern table
            let palette_low = ((pattern_low  >> shamt) & 0b1) |
                                (((pattern_high >> shamt) & 0b1) << 1);

            let palette_i = concat_palette_bits(palette_low, palette_high);
            let color = self.get_palette_color(palette_i);

            self.set_pixel(sprite_col + x, scanline, color);
        }

        SpriteRenderResult::Rendered
    }

    // requires that the bg for the scanline is already rendered
    // (due to priority)
    fn render_scanline_sprites(&mut self, scanline : u8) {
        debug_assert!(scanline < 240);

        // search for sprites where sprite.y + 1 <= scanline < sprite.y + 8 + 1

        let mut num_sprites = 0;

        for sprite_num in 0..64 {
            // sprites are delayed 1 scanline
            let y = self.oam[sprite_num*4];
            let visible = y < 0xF0;

            if visible && y + 1 <= scanline && scanline < y + 1 + 8 {
                self.render_sprite(sprite_num as u8, scanline);
                num_sprites += 1;
            }

            if num_sprites == 8 { break; }
        }
    }

    pub fn render(&mut self, picture : &mut super::graphics::Picture) {
        for i in 0..240 {
            self.render_scanline_bg(i);
            self.render_scanline_sprites(i);
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
