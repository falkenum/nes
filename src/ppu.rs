
use super::graphics::SCREEN_SIZE;
use super::{ Cartridge, RefCell, Rc };

pub struct PPU {
    pixeldata : [u8; SCREEN_SIZE],
    cart : Rc<RefCell<Cartridge>>,
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
    pub fn new(cart : Rc<RefCell<Cartridge>>) -> PPU {
        PPU {
            pixeldata : fill_color(0, 255, 255),
            cart : cart,
        }
    }
    pub fn render(&self, picture : &mut super::graphics::Picture) {
        picture.update(&self.pixeldata);
    }
}
