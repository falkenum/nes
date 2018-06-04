use super::{ PPU, Memory, Cartridge };
// use super::PALETTE_BGR;
use super::reg_id::*;

const SCREEN_WIDTH : usize = 256;

#[test]
fn attr_table_rendering() {
    // testing how attr table is read
    let mut p = PPU::new(Cartridge::test_ref());

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
        p.render_scanline(i);
    }

    // top left
    for i in 0..16 {
        let pixel = i;
        assert_eq!(p.pixeldata[pixel*3+0], 252);
        assert_eq!(p.pixeldata[pixel*3+1], 000);
        assert_eq!(p.pixeldata[pixel*3+2], 000);

        // bottom row of tile
        let pixel = 7*256+i;
        assert_eq!(p.pixeldata[pixel*3+0], 252);
        assert_eq!(p.pixeldata[pixel*3+1], 000);
        assert_eq!(p.pixeldata[pixel*3+2], 000);

        // second tile down
        let pixel = 8*256+i;
        assert_eq!(p.pixeldata[pixel*3+0], 252);
        assert_eq!(p.pixeldata[pixel*3+1], 000);
        assert_eq!(p.pixeldata[pixel*3+2], 000);
    }

    // top right
    for i in 16..32 {
        let pixel = i;
        assert_eq!(p.pixeldata[pixel*3+0], 032);
        assert_eq!(p.pixeldata[pixel*3+1], 000);
        assert_eq!(p.pixeldata[pixel*3+2], 168);

        // bottom row of tile
        let pixel = 7*256+i;
        assert_eq!(p.pixeldata[pixel*3+0], 032);
        assert_eq!(p.pixeldata[pixel*3+1], 000);
        assert_eq!(p.pixeldata[pixel*3+2], 168);

        // second tile down
        let pixel = 8*256+i;
        assert_eq!(p.pixeldata[pixel*3+0], 032);
        assert_eq!(p.pixeldata[pixel*3+1], 000);
        assert_eq!(p.pixeldata[pixel*3+2], 168);
    }

    // bottom left
    for i in 0..16 {
        let pixel = 16*256+i;
        assert_eq!(p.pixeldata[pixel*3+0], 000);
        assert_eq!(p.pixeldata[pixel*3+1], 120);
        assert_eq!(p.pixeldata[pixel*3+2], 000);

        // bottom row of tile
        let pixel = (16+7)*256+i;
        assert_eq!(p.pixeldata[pixel*3+0], 000);
        assert_eq!(p.pixeldata[pixel*3+1], 120);
        assert_eq!(p.pixeldata[pixel*3+2], 000);

        // second tile down
        let pixel = (16+8)*256+i;
        assert_eq!(p.pixeldata[pixel*3+0], 000);
        assert_eq!(p.pixeldata[pixel*3+1], 120);
        assert_eq!(p.pixeldata[pixel*3+2], 000);
    }

    // bottom right
    for i in 16..32 {
        let pixel = 16*256+i;
        assert_eq!(p.pixeldata[pixel*3+0], 088);
        assert_eq!(p.pixeldata[pixel*3+1], 064);
        assert_eq!(p.pixeldata[pixel*3+2], 000);

        // bottom row of tile
        let pixel = (16+7)*256+i;
        assert_eq!(p.pixeldata[pixel*3+0], 088);
        assert_eq!(p.pixeldata[pixel*3+1], 064);
        assert_eq!(p.pixeldata[pixel*3+2], 000);

        // second tile down
        let pixel = (16+8)*256+i;
        assert_eq!(p.pixeldata[pixel*3+0], 088);
        assert_eq!(p.pixeldata[pixel*3+1], 064);
        assert_eq!(p.pixeldata[pixel*3+2], 000);
    }

    p.render_scanline(239);
    // last tile group
    // top right, this last row is a 4*2 tile group
    for i in 240..256 {
        let pixel = 239*256+i;
        assert_eq!(p.pixeldata[pixel*3+0], 000);
        assert_eq!(p.pixeldata[pixel*3+1], 120);
        assert_eq!(p.pixeldata[pixel*3+2], 000);
    }
}

#[test]
fn nametable_rendering() {
    // testing how nametable is read
    let mut p = PPU::new(Cartridge::test_ref());

    p.mem.storeb(0x3F00, 0x00);
    p.mem.storeb(0x3F01, 0x01);
    p.mem.storeb(0x3F02, 0x02);
    p.mem.storeb(0x3F03, 0x03);

    p.mem.storeb(0x0000, 0x00);
    p.mem.storeb(0x0FF0, 0xFF);

    p.mem.storeb(0x2000, 0x00);
    p.mem.storeb(0x2001, 0xFF);

    p.render_scanline(0);
    for i in 0..8 {
        assert_eq!(p.pixeldata[i*3+0], 124);
        assert_eq!(p.pixeldata[i*3+1], 124);
        assert_eq!(p.pixeldata[i*3+2], 124);
    }
    for i in 8..16 {
        assert_eq!(p.pixeldata[i*3+0], 252);
        assert_eq!(p.pixeldata[i*3+1], 000);
        assert_eq!(p.pixeldata[i*3+2], 000);
    }

    p.mem.storeb(0x0AAF, 0xFF);
    p.mem.storeb(0x23BF, 0xAA);

    p.render_scanline(239);
    for i in 248..256 {
        assert_eq!(p.pixeldata[239*256*3+i*3+0], 188);
        assert_eq!(p.pixeldata[239*256*3+i*3+1], 000);
        assert_eq!(p.pixeldata[239*256*3+i*3+2], 000);
    }

    // TODO test other 3 nametables
}

#[test]
fn pattern_table_rendering() {
    // testing how a pattern gets data from palette ram
    let mut p = PPU::new(Cartridge::test_ref());

    p.mem.storeb(0x3F00, 0x00);
    p.mem.storeb(0x3F01, 0x01);
    p.mem.storeb(0x3F02, 0x02);
    p.mem.storeb(0x3F03, 0x03);

    // the pt, nt, and at are filled with 0's
    // the whole scanline rendered should be filled with the universal bg at 0x3F00
    p.render_scanline(0);
    for i in 0..SCREEN_WIDTH {
        assert_eq!(p.pixeldata[i*3+0], 124);
        assert_eq!(p.pixeldata[i*3+1], 124);
        assert_eq!(p.pixeldata[i*3+2], 124);
    }

    // color 1
    p.mem.storeb(0x0000, 0xFF);
    p.render_scanline(0);
    for i in 0..8 {
        assert_eq!(p.pixeldata[i*3+0], 252);
        assert_eq!(p.pixeldata[i*3+1], 000);
        assert_eq!(p.pixeldata[i*3+2], 000);
    }

    // color 3
    p.mem.storeb(0x0008, 0xFF);
    p.render_scanline(0);
    for i in 0..8 {
        assert_eq!(p.pixeldata[i*3+0], 188);
        assert_eq!(p.pixeldata[i*3+1], 040);
        assert_eq!(p.pixeldata[i*3+2], 068);
    }

    // color 2
    p.mem.storeb(0x0000, 0x00);
    p.render_scanline(0);
    for i in 0..8 {
        assert_eq!(p.pixeldata[i*3+0], 188);
        assert_eq!(p.pixeldata[i*3+1], 000);
        assert_eq!(p.pixeldata[i*3+2], 000);
    }

    // multiple colors in one row of a tile
    p.mem.storeb(0x0000, 0x0F);
    p.render_scanline(0);
    for i in 0..4 {
        assert_eq!(p.pixeldata[i*3+0], 188);
        assert_eq!(p.pixeldata[i*3+1], 000);
        assert_eq!(p.pixeldata[i*3+2], 000);
    }
    for i in 4..8 {
        assert_eq!(p.pixeldata[i*3+0], 188);
        assert_eq!(p.pixeldata[i*3+1], 040);
        assert_eq!(p.pixeldata[i*3+2], 068);
    }
}

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
    assert_eq!(p.address, 0x3F01);
}
