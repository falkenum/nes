use super::{ PPU, Memory, PALETTE_BGR };
use super::reg_id::*;

const SCREEN_WIDTH : usize = 256;

// TODO test 8x16 sprite, flipping, ignore pt base

// TODO test sprite overlap
// TODO test sprite and bg priority

#[test]
fn sprite_8x16_vert_flip() {
    let mut p = PPU::test();

    p.oam = [0xFF; 256];

    // 8x16 sprites
    p.control = 0x20;

    p.oam[0] = 0x00;
    p.oam[1] = 0x00;
    // vertical flip
    p.oam[2] = 0x80;
    p.oam[3] = 0x00;

    p.mem.storeb(0x0000, 0xFF);
    p.mem.storeb(0x001F, 0xFF);

    p.mem.storeb(0x3F00, 0x00);
    p.mem.storeb(0x3F11, 0x01);
    p.mem.storeb(0x3F12, 0x02);
    p.mem.storeb(0x3F13, 0x03);

    p.render_scanline_sprites(1);
    let sprite_start = 256;
    for i in 0..8 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[2].0);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[2].1);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[2].2);
    }

    p.render_scanline_sprites(16);
    let sprite_start = 256*16;
    for i in 0..8 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[1].0);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[1].1);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[1].2);
    }
}

#[test]
fn sprite_8x16_y() {
    let mut p = PPU::test();

    p.oam = [0xFF; 256];

    // 8x16 sprites
    p.control = 0x20;

    // change y
    p.oam[0] = 0x01;
    p.oam[1] = 0x00;
    p.oam[2] = 0x00;
    p.oam[3] = 0x00;

    p.mem.storeb(0x0000, 0xFF);
    p.mem.storeb(0x0018, 0xFF);

    p.mem.storeb(0x3F00, 0x00);
    p.mem.storeb(0x3F11, 0x01);
    p.mem.storeb(0x3F12, 0x02);
    p.mem.storeb(0x3F13, 0x03);

    // testing change in y
    p.render_scanline_sprites(2);
    let sprite_start = 256*2;
    for i in 0..8 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[1].0);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[1].1);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[1].2);
    }

    // sprite on last scanline
    p.oam[0] = 0xEE;
    p.render_scanline_sprites(239);
    let sprite_start = 256*239;
    for i in 0..8 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[1].0);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[1].1);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[1].2);
    }
}

#[test]
fn sprite_8x16_x() {
    let mut p = PPU::test();

    p.oam = [0xFF; 256];

    // 8x16 sprites
    p.control = 0x20;

    p.oam[0] = 0x00;
    p.oam[1] = 0x00;
    p.oam[2] = 0x00;
    p.oam[3] = 0x01;

    p.mem.storeb(0x0000, 0xFF);
    p.mem.storeb(0x0018, 0xFF);

    p.mem.storeb(0x3F00, 0x00);
    p.mem.storeb(0x3F11, 0x01);
    p.mem.storeb(0x3F12, 0x02);
    p.mem.storeb(0x3F13, 0x03);

    p.render_scanline_sprites(1);
    let sprite_start = 256 + 1;

    for i in 0..8 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[1].0);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[1].1);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[1].2);
    }

    // last column
    p.oam[3] = 0xFF;
    p.render_scanline_sprites(1);
    let sprite_start = 256 + 255;

    assert_eq!(p.pixeldata[sprite_start*3+0], PALETTE_BGR[1].0);
    assert_eq!(p.pixeldata[sprite_start*3+1], PALETTE_BGR[1].1);
    assert_eq!(p.pixeldata[sprite_start*3+2], PALETTE_BGR[1].2);
}

#[test]
fn sprite_8x16_tile() {
    let mut p = PPU::test();

    p.oam = [0xFF; 256];

    // 8x16 sprites
    p.control = 0x20;

    p.oam[0] = 0x00;
    p.oam[1] = 0x02;
    p.oam[2] = 0x00;
    p.oam[3] = 0x00;

    p.mem.storeb(0x0000, 0xFF);
    p.mem.storeb(0x0018, 0xFF);

    p.mem.storeb(0x0020, 0xFF);
    p.mem.storeb(0x0028, 0xFF);
    p.mem.storeb(0x0030, 0xFF);

    p.mem.storeb(0x3F00, 0x00);
    p.mem.storeb(0x3F11, 0x01);
    p.mem.storeb(0x3F12, 0x02);
    p.mem.storeb(0x3F13, 0x03);

    p.render_scanline_sprites(1);
    let sprite_start = 256;

    for i in 0..8 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[3].0);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[3].1);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[3].2);
    }

    p.render_scanline_sprites(9);
    let sprite_start = 256*9;

    for i in 0..8 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[1].0);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[1].1);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[1].2);
    }

    p.oam[1] = 0xFE;

    p.mem.storeb(0x0FE8, 0xFF);
    p.mem.storeb(0x0FF8, 0xFF);

    p.render_scanline_sprites(1);
    let sprite_start = 256;

    for i in 0..8 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[2].0);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[2].1);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[2].2);
    }

    p.render_scanline_sprites(9);
    let sprite_start = 256*9;

    for i in 0..8 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[2].0);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[2].1);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[2].2);
    }
}

#[test]
fn sprite_8x16_pt_base() {
    let mut p = PPU::test();

    p.oam = [0xFF; 256];

    // 8x16 sprites
    p.control = 0x20;

    p.oam[0] = 0x00;
    p.oam[1] = 0x01;
    p.oam[2] = 0x00;
    p.oam[3] = 0x00;

    p.mem.storeb(0x0000, 0xFF);
    p.mem.storeb(0x0018, 0xFF);

    p.mem.storeb(0x1000, 0xFF);
    p.mem.storeb(0x1008, 0xFF);
    p.mem.storeb(0x1010, 0xFF);

    p.mem.storeb(0x3F00, 0x00);
    p.mem.storeb(0x3F11, 0x01);
    p.mem.storeb(0x3F12, 0x02);
    p.mem.storeb(0x3F13, 0x03);

    p.render_scanline_sprites(1);
    let sprite_start = 256;

    for i in 0..8 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[3].0);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[3].1);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[3].2);
    }

    p.render_scanline_sprites(9);
    let sprite_start = 256*9;

    for i in 0..8 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[1].0);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[1].1);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[1].2);
    }
}

#[test]
fn sprite_8x16_basic() {
    let mut p = PPU::test();

    p.oam = [0xFF; 256];

    // 8x16 sprites
    p.control = 0x20;

    p.oam[0] = 0x00;
    p.oam[1] = 0x00;
    p.oam[2] = 0x00;
    p.oam[3] = 0x00;

    p.mem.storeb(0x0000, 0xFF);
    p.mem.storeb(0x0018, 0xFF);

    p.mem.storeb(0x3F00, 0x00);
    p.mem.storeb(0x3F11, 0x01);
    p.mem.storeb(0x3F12, 0x02);
    p.mem.storeb(0x3F13, 0x03);

    p.render_scanline_sprites(1);
    let sprite_start = 256;

    for i in 0..8 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[1].0);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[1].1);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[1].2);
    }

    p.render_scanline_sprites(9);
    let sprite_start = 256*9;

    for i in 0..8 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[2].0);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[2].1);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[2].2);
    }
}

#[test]
fn sprite_pt_base() {
    let mut p = PPU::test();

    p.oam = [0xFF; 256];

    // pt base 0x1000 for 8x8 sprites
    p.control = 0x08;

    p.oam[0] = 0x00;
    p.oam[1] = 0x00;
    p.oam[2] = 0x00;
    p.oam[3] = 0x00;

    p.mem.storeb(0x0000, 0xFF);
    p.mem.storeb(0x1008, 0xFF);

    p.mem.storeb(0x3F00, 0x00);

    p.mem.storeb(0x3F11, 0x01);
    p.mem.storeb(0x3F12, 0x02);
    p.mem.storeb(0x3F13, 0x03);

    p.render_scanline_sprites(1);
    let sprite_start = 256;

    for i in 0..8 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[2].0);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[2].1);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[2].2);
    }
}

#[test]
fn multiple_sprites() {
    let mut p = PPU::test();

    p.oam = [0xFF; 256];

    // 8 sprites, each with 8 pixels in between of background

    for i in 0..8 {
        p.oam[i*4+0] = 0x00;
        p.oam[i*4+1] = 0x00;
        p.oam[i*4+2] = 0x00;
        p.oam[i*4+3] = 16 * i as u8;
    }

    p.mem.storeb(0x0000, 0xFF);

    p.mem.storeb(0x3F00, 0x00);
    p.mem.storeb(0x3F01, 0x01);
    p.mem.storeb(0x3F11, 0x11);
    p.mem.storeb(0x3F12, 0x12);
    p.mem.storeb(0x3F13, 0x13);

    p.render_scanline_sprites(1);

    for sprite in 0..8 {
        let sprite_start = 256 + sprite*16;
        for i in 0..8 {
            assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[0x11].0);
            assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[0x11].1);
            assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[0x11].2);

        }

        // check that the in between part is unmodified
        assert_eq!(p.pixeldata[(sprite_start + 9)*3], 000);
    }

    p.oam[9*4+0] = 0x00;
    p.oam[9*4+1] = 0x00;
    p.oam[9*4+2] = 0x00;
    p.oam[9*4+3] = 16 * 9 as u8;

    p.render_scanline_sprites(1);

    for sprite in 0..8 {
        let sprite_start = 256 + sprite*16;
        for i in 0..8 {
            assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[0x11].0);
            assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[0x11].1);
            assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[0x11].2);

        }

        // check that the in between part is unmodified
        assert_eq!(p.pixeldata[(sprite_start + 9)*3], 000);
    }

    let sprite_start = 256 + 9*16;
    // 9th sprite should not be rendered
    assert_eq!(p.pixeldata[sprite_start*3+0], 000);
    assert_eq!(p.pixeldata[sprite_start*3+1], 000);
    assert_eq!(p.pixeldata[sprite_start*3+2], 000);
}

#[test]
fn sprite_and_bg_rendering() {
    let mut p = PPU::test();

    p.oam = [0xFF; 256];

    p.oam[0] = 0x00;
    p.oam[1] = 0x01;
    p.oam[2] = 0x00;
    p.oam[3] = 0x00;

    p.mem.storeb(0x0001, 0xFF);
    p.mem.storeb(0x0018, 0xFF);

    p.mem.storeb(0x3F00, 0x00);
    p.mem.storeb(0x3F01, 0x01);

    p.mem.storeb(0x3F11, 0x11);
    p.mem.storeb(0x3F12, 0x12);
    p.mem.storeb(0x3F13, 0x13);

    p.render_scanline_bg(1);
    let pixel = 256;
    for i in 0..8 {
        assert_eq!(p.pixeldata[(pixel + i)*3+0], PALETTE_BGR[1].0);
        assert_eq!(p.pixeldata[(pixel + i)*3+1], PALETTE_BGR[1].1);
        assert_eq!(p.pixeldata[(pixel + i)*3+2], PALETTE_BGR[1].2);
    }

    p.render_scanline_sprites(1);
    let sprite_start = 256;
    for i in 0..8 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[0x12].0);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[0x12].1);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[0x12].2);
    }

    // bg after the sprite should be unchanged
    assert_eq!(p.pixeldata[(pixel + 9)*3+0], 252);

}

#[test]
fn sprite_tiles() {
    let mut p = PPU::test();

    p.oam = [0xFF; 256];

    p.oam[0] = 0x00;
    p.oam[1] = 0x01;
    p.oam[2] = 0x00;
    p.oam[3] = 0x00;

    p.mem.storeb(0x0000, 0xFF);
    p.mem.storeb(0x0018, 0xFF);

    p.mem.storeb(0x3F00, 0x00);
    p.mem.storeb(0x3F11, 0x01);
    p.mem.storeb(0x3F12, 0x02);
    p.mem.storeb(0x3F13, 0x03);

    p.render_scanline_sprites(1);
    let sprite_start = 256;
    for i in 0..8 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[2].0);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[2].1);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[2].2);
    }
}

#[test]
fn sprite_palette() {
    let mut p = PPU::test();

    p.oam = [0xFF; 256];

    p.oam[0] = 0x00;
    p.oam[1] = 0x00;
    p.oam[2] = 0x01;
    p.oam[3] = 0x00;

    p.mem.storeb(0x0000, 0xFF);

    p.mem.storeb(0x3F00, 0x00);
    p.mem.storeb(0x3F11, 0x01);
    p.mem.storeb(0x3F12, 0x02);
    p.mem.storeb(0x3F13, 0x03);
    p.mem.storeb(0x3F15, 0x05);
    p.mem.storeb(0x3F16, 0x06);
    p.mem.storeb(0x3F17, 0x07);

    p.render_scanline_sprites(1);
    let sprite_start = 256;
    for i in 0..8 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[5].0);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[5].1);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[5].2);
    }
}

#[test]
fn sprite_horiz_flip() {
    let mut p = PPU::test();

    p.oam = [0xFF; 256];

    p.oam[0] = 0x00;
    p.oam[1] = 0x00;

    // horizontal flip
    p.oam[2] = 0x40;

    p.oam[3] = 0x00;

    p.mem.storeb(0x0000, 0x0F);
    p.mem.storeb(0x0008, 0xF0);

    p.mem.storeb(0x3F00, 0x00);
    p.mem.storeb(0x3F11, 0x01);
    p.mem.storeb(0x3F12, 0x02);
    p.mem.storeb(0x3F13, 0x03);

    p.render_scanline_sprites(1);
    let sprite_start = 256;
    for i in 0..4 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[1].0);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[1].1);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[1].2);
    }
    for i in 4..8 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[2].0);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[2].1);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[2].2);
    }

    // horizontal flip at the last col
    p.oam[3] = 0xFF;
    p.render_scanline_sprites(1);
    let sprite_start = 256 + 255;
    assert_eq!(p.pixeldata[sprite_start*3+0], PALETTE_BGR[1].0);
    assert_eq!(p.pixeldata[sprite_start*3+1], PALETTE_BGR[1].1);
    assert_eq!(p.pixeldata[sprite_start*3+2], PALETTE_BGR[1].2);
}

#[test]
fn sprite_vert_flip() {
    let mut p = PPU::test();

    p.oam = [0xFF; 256];

    p.oam[0] = 0x00;
    p.oam[1] = 0x00;

    // vertical flip
    p.oam[2] = 0x80;

    p.oam[3] = 0x00;

    p.mem.storeb(0x0000, 0xFF);
    p.mem.storeb(0x000F, 0xFF);

    p.mem.storeb(0x3F00, 0x00);
    p.mem.storeb(0x3F11, 0x01);
    p.mem.storeb(0x3F12, 0x02);
    p.mem.storeb(0x3F13, 0x03);

    p.render_scanline_sprites(1);
    let sprite_start = 256;
    for i in 0..8 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[2].0);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[2].1);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[2].2);
    }

    p.render_scanline_sprites(8);
    let sprite_start = 256*8;
    for i in 0..8 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[1].0);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[1].1);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[1].2);
    }
}

#[test]
fn sprite_x() {
    let mut p = PPU::test();

    p.oam = [0xFF; 256];

    p.oam[0] = 0x00;
    p.oam[1] = 0x00;
    p.oam[2] = 0x00;
    // change x
    p.oam[3] = 0x01;

    p.mem.storeb(0x0000, 0xFF);
    p.mem.storeb(0x000F, 0xFF);

    p.mem.storeb(0x3F00, 0x00);

    p.mem.storeb(0x3F11, 0x01);
    p.mem.storeb(0x3F12, 0x02);
    p.mem.storeb(0x3F13, 0x03);

    p.render_scanline_sprites(1);
    let sprite_start = 256 + 1;
    for i in 0..8 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[1].0);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[1].1);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[1].2);
    }

    // last column
    p.oam[3] = 0xFF;
    p.render_scanline_sprites(1);
    let sprite_start = 256 + 255;
    assert_eq!(p.pixeldata[sprite_start*3+0], PALETTE_BGR[1].0);
    assert_eq!(p.pixeldata[sprite_start*3+1], PALETTE_BGR[1].1);
    assert_eq!(p.pixeldata[sprite_start*3+2], PALETTE_BGR[1].2);
}

#[test]
fn sprite_y() {
    let mut p = PPU::test();

    p.oam = [0xFF; 256];

    // change y
    p.oam[0] = 0x01;
    p.oam[1] = 0x00;
    p.oam[2] = 0x00;
    p.oam[3] = 0x00;

    p.mem.storeb(0x0000, 0xFF);
    p.mem.storeb(0x000F, 0xFF);

    p.mem.storeb(0x3F00, 0x00);

    p.mem.storeb(0x3F11, 0x01);
    p.mem.storeb(0x3F12, 0x02);
    p.mem.storeb(0x3F13, 0x03);

    // this scanline shouldn't have anything
    p.render_scanline_sprites(1);
    let sprite_start = 256;
    for i in 0..8 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], 000);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], 000);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], 000);
    }

    // testing change in y
    p.render_scanline_sprites(2);
    let sprite_start = 256*2;
    for i in 0..8 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[1].0);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[1].1);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[1].2);
    }

    // sprite on last scanline
    p.oam[0] = 0xEE;
    p.render_scanline_sprites(239);
    let sprite_start = 256*239;
    for i in 0..8 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[1].0);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[1].1);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[1].2);
    }
}

#[test]
fn basic_sprite_rendering() {
    let mut p = PPU::test();

    p.oam = [0xFF; 256];

    p.oam[0] = 0x00;
    p.oam[1] = 0x00;
    p.oam[2] = 0x00;
    p.oam[3] = 0x00;

    p.mem.storeb(0x0000, 0xFF);
    p.mem.storeb(0x000F, 0xFF);

    p.mem.storeb(0x3F00, 0x00);

    p.mem.storeb(0x3F11, 0x01);
    p.mem.storeb(0x3F12, 0x02);
    p.mem.storeb(0x3F13, 0x03);

    p.render_scanline_sprites(1);
    let sprite_start = 256;

    for i in 0..8 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[1].0);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[1].1);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[1].2);
    }

    // check that only 8 pixels were changed
    assert_eq!(p.pixeldata[(sprite_start + 8)*3+0], 000);

    // bottom row of sprite
    p.render_scanline_sprites(8);
    let sprite_start = 256*8;

    for i in 0..8 {
        assert_eq!(p.pixeldata[(sprite_start + i)*3+0], PALETTE_BGR[2].0);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+1], PALETTE_BGR[2].1);
        assert_eq!(p.pixeldata[(sprite_start + i)*3+2], PALETTE_BGR[2].2);
    }

    // check that only 8 pixels were changed
    assert_eq!(p.pixeldata[(sprite_start + 8)*3+0], 000);
}

#[test]
fn oam_regs() {
    let mut p = PPU::test();
    p.reg_write(OAMADDR, 0x00);
    p.reg_write(OAMDATA, 0xFF);

    assert_eq!(p.oam_addr, 0x01);
    assert_eq!(p.oam[0x00], 0xFF);

    // reads shouldn't increment oam_addr
    assert_eq!(p.oam_addr, 0x01);

    p.reg_write(OAMDATA, 0xFF);
    assert_eq!(p.oam[0x01], 0xFF);

    p.reg_write(OAMADDR, 0xFF);
    p.reg_write(OAMDATA, 0xFF);
    assert_eq!(p.oam_addr, 0x00);
    assert_eq!(p.oam[0xFF], 0xFF);
}

#[test]
fn bg_pt_base() {
    // testing bit 4 of ppuctrl
    let mut p = PPU::test();

    p.mem.storeb(0x3F01, 0x01);
    p.mem.storeb(0x3F02, 0x02);

    p.mem.storeb(0x0008, 0x80);
    p.mem.storeb(0x1000, 0x80);

    // use pt at 0x0000
    p.control = 0x00;

    p.render_scanline_bg(0);

    let pixel = 0;
    assert_eq!(p.pixeldata[pixel*3+0], PALETTE_BGR[2].0);
    assert_eq!(p.pixeldata[pixel*3+1], PALETTE_BGR[2].1);
    assert_eq!(p.pixeldata[pixel*3+2], PALETTE_BGR[2].2);

    // use pt at 0x1000
    p.control = 0x10;

    p.render_scanline_bg(0);

    let pixel = 0;
    assert_eq!(p.pixeldata[pixel*3+0], PALETTE_BGR[1].0);
    assert_eq!(p.pixeldata[pixel*3+1], PALETTE_BGR[1].1);
    assert_eq!(p.pixeldata[pixel*3+2], PALETTE_BGR[1].2);
}

#[test]
fn palette_mirroring() {
    // testing palette mirroring
    let mut p = PPU::test();
    p.mem.storeb(0x3F00, 0x00);
    // p.mem.storeb(0x3F01, 0x01);
    // p.mem.storeb(0x3F02, 0x02);
    // p.mem.storeb(0x3F03, 0x03);

    p.mem.storeb(0x3F04, 0x04);
    p.mem.storeb(0x3F05, 0x05);
    p.mem.storeb(0x3F08, 0x08);
    p.mem.storeb(0x3F09, 0x09);

    p.mem.storeb(0x3F15, 0x15);
    p.mem.storeb(0x3F16, 0x16);
    p.mem.storeb(0x3F17, 0x17);


    assert_eq!(p.mem.loadb(0x3F04), 0x04);
    assert_eq!(p.mem.loadb(0x3F05), 0x05);
    assert_eq!(p.mem.loadb(0x3F08), 0x08);
    assert_eq!(p.mem.loadb(0x3F09), 0x09);

    assert_eq!(p.mem.loadb(0x3F14), 0x04);
    assert_eq!(p.mem.loadb(0x3F15), 0x15);
    assert_eq!(p.mem.loadb(0x3F16), 0x16);
    assert_eq!(p.mem.loadb(0x3F17), 0x17);

    assert_eq!(p.mem.loadb(0x3F18), 0x08);
    p.mem.storeb(0x3F18, 0x18);
    assert_eq!(p.mem.loadb(0x3F18), 0x18);
    assert_eq!(p.mem.loadb(0x3F08), 0x18);

    p.mem.storeb(0x3F10, 0xFF);
    assert_eq!(p.mem.loadb(0x3F00), 0xFF);
}

#[test]
fn palette_bg_color() {
    // testing the behavior of 0's in a pattern
    let mut p = PPU::test();

    p.mem.storeb(0x3F00, 0x00);
    p.mem.storeb(0x3F04, 0x01);
    p.mem.storeb(0x3F05, 0x02);

    p.mem.storeb(0x0000, 0xF0);
    p.mem.storeb(0x23C0, 0b0000_00_01);

    p.render_scanline_bg(0);

    let pixel = 0;
    assert_eq!(p.pixeldata[pixel*3+0], PALETTE_BGR[2].0);
    assert_eq!(p.pixeldata[pixel*3+1], PALETTE_BGR[2].1);
    assert_eq!(p.pixeldata[pixel*3+2], PALETTE_BGR[2].2);

    let pixel = 4;
    assert_eq!(p.pixeldata[pixel*3+0], PALETTE_BGR[0].0);
    assert_eq!(p.pixeldata[pixel*3+1], PALETTE_BGR[0].1);
    assert_eq!(p.pixeldata[pixel*3+2], PALETTE_BGR[0].2);
}

#[test]
fn vram_inc() {
    // testing bit 2 of control reg
    let mut p = PPU::test();

    p.reg_write(CONTROL, 0b0000_0000);
    assert_eq!(p.address, 0x0000);
    p.reg_write(DATA, 0xFF);
    assert_eq!(p.address, 0x0001);

    p.reg_write(CONTROL, 0b0000_0100);
    p.reg_write(DATA, 0xFF);
    assert_eq!(p.address, 0x0021);

    p.reg_read(DATA);
    assert_eq!(p.address, 0x0041);

    p.reg_write(CONTROL, 0b0000_0000);
    p.reg_read(DATA);
    assert_eq!(p.address, 0x0042);
}

#[test]
fn nametable_choice() {
    // testing bits 0-1 of control reg

    let mut p = PPU::test();

    p.mem.storeb(0x3F00, 0x00);
    p.mem.storeb(0x3F01, 0x01);

    p.mem.storeb(0x0010, 0xFF);
    p.mem.storeb(0x0020, 0x00);

    p.mem.storeb(0x2000, 0x01);
    p.mem.storeb(0x2400, 0x02);

    let pixel = 0;

    p.reg_write(CONTROL, 0x00);
    p.render_scanline_bg(0);
    assert_eq!(p.pixeldata[pixel*3+0], PALETTE_BGR[1].0);
    assert_eq!(p.pixeldata[pixel*3+1], PALETTE_BGR[1].1);
    assert_eq!(p.pixeldata[pixel*3+2], PALETTE_BGR[1].2);

    p.reg_write(CONTROL, 0x01);
    p.render_scanline_bg(0);
    assert_eq!(p.pixeldata[pixel*3+0], PALETTE_BGR[0].0);
    assert_eq!(p.pixeldata[pixel*3+1], PALETTE_BGR[0].1);
    assert_eq!(p.pixeldata[pixel*3+2], PALETTE_BGR[0].2);
}

#[test]
fn attr_table_rendering() {
    // testing how attr table is read
    let mut p = PPU::test();

    p.mem.storeb(0x3F00, 0x00);
    p.mem.storeb(0x3F01, 0x01);
    p.mem.storeb(0x3F02, 0x02);
    p.mem.storeb(0x3F03, 0x03);

    p.mem.storeb(0x3F05, 0x05);
    p.mem.storeb(0x3F09, 0x09);
    p.mem.storeb(0x3F0D, 0x0C);

    p.mem.storeb(0x23C0, 0b11_10_01_00);
    p.mem.storeb(0x23FF, 0b00_01_10_11);


    // fill tile with 1's
    for i in 0..8 {
        p.mem.storeb(0x0000 + i, 0xFF);
    }

    // four rows of tiles
    for i in 0..32 {
        p.render_scanline_bg(i);
    }

    // top left
    for i in 0..16 {
        let pixel = i;
        assert_eq!(p.pixeldata[pixel*3+0], PALETTE_BGR[1].0);
        assert_eq!(p.pixeldata[pixel*3+1], PALETTE_BGR[1].1);
        assert_eq!(p.pixeldata[pixel*3+2], PALETTE_BGR[1].2);

        // bottom row of tile
        let pixel = 7*256+i;
        assert_eq!(p.pixeldata[pixel*3+0], PALETTE_BGR[1].0);
        assert_eq!(p.pixeldata[pixel*3+1], PALETTE_BGR[1].1);
        assert_eq!(p.pixeldata[pixel*3+2], PALETTE_BGR[1].2);

        // second tile down
        let pixel = 8*256+i;
        assert_eq!(p.pixeldata[pixel*3+0], PALETTE_BGR[1].0);
        assert_eq!(p.pixeldata[pixel*3+1], PALETTE_BGR[1].1);
        assert_eq!(p.pixeldata[pixel*3+2], PALETTE_BGR[1].2);
    }

    // top right
    for i in 16..32 {
        let pixel = i;
        assert_eq!(p.pixeldata[pixel*3+0], PALETTE_BGR[5].0);
        assert_eq!(p.pixeldata[pixel*3+1], PALETTE_BGR[5].1);
        assert_eq!(p.pixeldata[pixel*3+2], PALETTE_BGR[5].2);

        // bottom row of tile
        let pixel = 7*256+i;
        assert_eq!(p.pixeldata[pixel*3+0], PALETTE_BGR[5].0);
        assert_eq!(p.pixeldata[pixel*3+1], PALETTE_BGR[5].1);
        assert_eq!(p.pixeldata[pixel*3+2], PALETTE_BGR[5].2);

        // second tile down
        let pixel = 8*256+i;
        assert_eq!(p.pixeldata[pixel*3+0], PALETTE_BGR[5].0);
        assert_eq!(p.pixeldata[pixel*3+1], PALETTE_BGR[5].1);
        assert_eq!(p.pixeldata[pixel*3+2], PALETTE_BGR[5].2);
    }

    // bottom left
    for i in 0..16 {
        let pixel = 16*256+i;
        assert_eq!(p.pixeldata[pixel*3+0], PALETTE_BGR[9].0);
        assert_eq!(p.pixeldata[pixel*3+1], PALETTE_BGR[9].1);
        assert_eq!(p.pixeldata[pixel*3+2], PALETTE_BGR[9].2);

        // bottom row of tile
        let pixel = (16+7)*256+i;
        assert_eq!(p.pixeldata[pixel*3+0], PALETTE_BGR[9].0);
        assert_eq!(p.pixeldata[pixel*3+1], PALETTE_BGR[9].1);
        assert_eq!(p.pixeldata[pixel*3+2], PALETTE_BGR[9].2);

        // second tile down
        let pixel = (16+8)*256+i;
        assert_eq!(p.pixeldata[pixel*3+0], PALETTE_BGR[9].0);
        assert_eq!(p.pixeldata[pixel*3+1], PALETTE_BGR[9].1);
        assert_eq!(p.pixeldata[pixel*3+2], PALETTE_BGR[9].2);
    }

    // bottom right
    for i in 16..32 {
        let pixel = 16*256+i;
        assert_eq!(p.pixeldata[pixel*3+0], PALETTE_BGR[0xC].0);
        assert_eq!(p.pixeldata[pixel*3+1], PALETTE_BGR[0xC].1);
        assert_eq!(p.pixeldata[pixel*3+2], PALETTE_BGR[0xC].2);

        // bottom row of tile
        let pixel = (16+7)*256+i;
        assert_eq!(p.pixeldata[pixel*3+0], PALETTE_BGR[0xC].0);
        assert_eq!(p.pixeldata[pixel*3+1], PALETTE_BGR[0xC].1);
        assert_eq!(p.pixeldata[pixel*3+2], PALETTE_BGR[0xC].2);

        // second tile down
        let pixel = (16+8)*256+i;
        assert_eq!(p.pixeldata[pixel*3+0], PALETTE_BGR[0xC].0);
        assert_eq!(p.pixeldata[pixel*3+1], PALETTE_BGR[0xC].1);
        assert_eq!(p.pixeldata[pixel*3+2], PALETTE_BGR[0xC].2);
    }

    p.render_scanline_bg(239);
    // last tile group
    // top right, this last row is a 4*2 tile group
    for i in 240..256 {
        let pixel = 239*256+i;
        assert_eq!(p.pixeldata[pixel*3+0], PALETTE_BGR[9].0);
        assert_eq!(p.pixeldata[pixel*3+1], PALETTE_BGR[9].1);
        assert_eq!(p.pixeldata[pixel*3+2], PALETTE_BGR[9].2);
    }
}

#[test]
fn nametable_rendering() {
    // testing how nametable is read
    let mut p = PPU::test();

    p.mem.storeb(0x3F00, 0x00);
    p.mem.storeb(0x3F01, 0x01);
    p.mem.storeb(0x3F02, 0x02);
    p.mem.storeb(0x3F03, 0x03);

    p.mem.storeb(0x0000, 0x00);
    p.mem.storeb(0x0FF0, 0xFF);

    p.mem.storeb(0x2000, 0x00);
    p.mem.storeb(0x2001, 0xFF);

    p.render_scanline_bg(0);
    for i in 0..8 {
        assert_eq!(p.pixeldata[i*3+0], PALETTE_BGR[0].0);
        assert_eq!(p.pixeldata[i*3+1], PALETTE_BGR[0].1);
        assert_eq!(p.pixeldata[i*3+2], PALETTE_BGR[0].2);
    }
    for i in 8..16 {
        assert_eq!(p.pixeldata[i*3+0], PALETTE_BGR[1].0);
        assert_eq!(p.pixeldata[i*3+1], PALETTE_BGR[1].1);
        assert_eq!(p.pixeldata[i*3+2], PALETTE_BGR[1].2);
    }

    p.mem.storeb(0x0AAF, 0xFF);
    p.mem.storeb(0x23BF, 0xAA);

    p.render_scanline_bg(239);
    for i in 248..256 {
        assert_eq!(p.pixeldata[239*256*3+i*3+0], PALETTE_BGR[2].0);
        assert_eq!(p.pixeldata[239*256*3+i*3+1], PALETTE_BGR[2].1);
        assert_eq!(p.pixeldata[239*256*3+i*3+2], PALETTE_BGR[2].2);
    }

    // TODO test other 3 nametables
}

#[test]
fn pattern_table_rendering() {
    // testing how a pattern gets data from palette ram
    let mut p = PPU::test();

    p.mem.storeb(0x3F00, 0x00);
    p.mem.storeb(0x3F01, 0x01);
    p.mem.storeb(0x3F02, 0x02);
    p.mem.storeb(0x3F03, 0x03);

    // the pt, nt, and at are filled with 0's
    // the whole scanline rendered should be filled with the universal bg at 0x3F00
    p.render_scanline_bg(0);
    for i in 0..SCREEN_WIDTH {
        assert_eq!(p.pixeldata[i*3+0], PALETTE_BGR[0].0);
        assert_eq!(p.pixeldata[i*3+1], PALETTE_BGR[0].1);
        assert_eq!(p.pixeldata[i*3+2], PALETTE_BGR[0].2);
    }

    // color 1
    p.mem.storeb(0x0000, 0xFF);
    p.render_scanline_bg(0);
    for i in 0..8 {
        assert_eq!(p.pixeldata[i*3+0], PALETTE_BGR[1].0);
        assert_eq!(p.pixeldata[i*3+1], PALETTE_BGR[1].1);
        assert_eq!(p.pixeldata[i*3+2], PALETTE_BGR[1].2);
    }

    // color 3
    p.mem.storeb(0x0008, 0xFF);
    p.render_scanline_bg(0);
    for i in 0..8 {
        assert_eq!(p.pixeldata[i*3+0], PALETTE_BGR[3].0);
        assert_eq!(p.pixeldata[i*3+1], PALETTE_BGR[3].1);
        assert_eq!(p.pixeldata[i*3+2], PALETTE_BGR[3].2);
    }

    // color 2
    p.mem.storeb(0x0000, 0x00);
    p.render_scanline_bg(0);
    for i in 0..8 {
        assert_eq!(p.pixeldata[i*3+0], PALETTE_BGR[2].0);
        assert_eq!(p.pixeldata[i*3+1], PALETTE_BGR[2].1);
        assert_eq!(p.pixeldata[i*3+2], PALETTE_BGR[2].2);
    }

    // multiple colors in one row of a tile
    p.mem.storeb(0x0000, 0x0F);
    p.render_scanline_bg(0);
    for i in 0..4 {
        assert_eq!(p.pixeldata[i*3+0], PALETTE_BGR[2].0);
        assert_eq!(p.pixeldata[i*3+1], PALETTE_BGR[2].1);
        assert_eq!(p.pixeldata[i*3+2], PALETTE_BGR[2].2);
    }
    for i in 4..8 {
        assert_eq!(p.pixeldata[i*3+0], PALETTE_BGR[3].0);
        assert_eq!(p.pixeldata[i*3+1], PALETTE_BGR[3].1);
        assert_eq!(p.pixeldata[i*3+2], PALETTE_BGR[3].2);
    }
}

#[test]
fn mappings() {
    let mut p = PPU::test();
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
    let mut p = PPU::test();
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
    let mut p = PPU::test();
    assert_eq!(p.address, 0x0000);

    p.reg_write(ADDRESS, 0x3F);
    assert_eq!(p.address, 0x0000);
    p.reg_write(ADDRESS, 0x00);
    assert_eq!(p.address, 0x3F00);

    p.reg_write(DATA, 0xFF);

    assert_eq!(p.mem.loadb(0x3F00), 0xFF);
    assert_eq!(p.address, 0x3F01);

    p.reg_write(ADDRESS, 0x3F);
    p.reg_write(ADDRESS, 0x00);
    assert_eq!(p.reg_read(DATA), 0xFF);
    assert_eq!(p.address, 0x3F01);

    p.reg_write(ADDRESS, 0x20);
    p.reg_write(ADDRESS, 0x00);
    p.reg_write(DATA, 0x00);
    p.reg_write(DATA, 0x01);
    p.reg_write(DATA, 0x02);
    assert_eq!(p.address, 0x2003);

    p.reg_write(ADDRESS, 0x20);
    p.reg_write(ADDRESS, 0x00);

    // toss read buffer
    p.reg_read(DATA);

    assert_eq!(p.reg_read(DATA), 0x00);
    assert_eq!(p.reg_read(DATA), 0x01);
    assert_eq!(p.reg_read(DATA), 0x02);
    assert_eq!(p.address, 0x2004);
}
