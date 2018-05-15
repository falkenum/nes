
use super::graphics::SCREEN_SIZE;
use super::{ Component, Memory, Cartridge };

pub struct PPU {
    pixeldata : [u8; SCREEN_SIZE],
    mem : PPUMem,
}

enum MirroringType {
    Vertical,
    Horizontal,
}

const VRAM_SIZE : u16 = 0x0800;
const VRAM_FIRST : u16 = 0x2000;
const VRAM_LAST : u16 = 0x3EFF;
const PALETTE_RAM_SIZE : u16 = 0x0020;
const PALETTE_RAM_FIRST : u16 = 0x3F00;
const PALETTE_RAM_LAST : u16 = 0x3FFF;
const CART_FIRST : u16 = 0x0000;
const CART_LAST : u16 = 0x1FFF;

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
    cart : Component<Cartridge>,
    vram : [u8; VRAM_SIZE as usize],
    palette_ram : [u8; PALETTE_RAM_SIZE as usize],
}


// TODO right now only vertical mirroring is suppported
// need to update the impl Memory to index into vram appropriately for
// horizontal mirroring.
impl Memory for PPUMem {
    fn loadb(&self, addr : u16) -> u8 {
        match addr {
            CART_FIRST...CART_LAST => self.cart.borrow().loadb(addr),
            VRAM_FIRST...VRAM_LAST => self.vram[(addr % VRAM_SIZE) as usize],
            PALETTE_RAM_FIRST...PALETTE_RAM_LAST =>
                self.palette_ram[(addr % PALETTE_RAM_SIZE) as usize],
            _ => panic!("invalid ppu address"),
        }
    }
    fn storeb(&mut self, addr : u16, val : u8) {
        match addr {
            // CART_FIRST...CART_LAST => self.cart.borrow_mut().storeb(addr, val),
            VRAM_FIRST...VRAM_LAST => self.vram[(addr % VRAM_SIZE) as usize] = val,
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
    pub fn new(cart : Component<Cartridge>) -> PPU {
        PPU {
            pixeldata : fill_color(0, 255, 255),
            mem : PPUMem {
                cart : cart,
                vram : [0; VRAM_SIZE as usize],
                palette_ram : [0; PALETTE_RAM_SIZE as usize],
            }
        }
    }
    pub fn render(&self, picture : &mut super::graphics::Picture) {
        picture.update(&self.pixeldata);
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
}
