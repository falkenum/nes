use super::{ PPU, Memory, Cartridge };
// use super::PALETTE_BGR;
use super::reg_id::*;

const SCREEN_WIDTH : usize = 256;

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
        assert_eq!(p.pixeldata[i*3 + 0], 124);
        assert_eq!(p.pixeldata[i*3 + 1], 124);
        assert_eq!(p.pixeldata[i*3 + 2], 124);
    }

    // color 1
    p.mem.storeb(0x0000, 0xFF);
    p.render_scanline(0);
    for i in 0..8 {
        assert_eq!(p.pixeldata[i*3 + 0], 252);
        assert_eq!(p.pixeldata[i*3 + 1], 000);
        assert_eq!(p.pixeldata[i*3 + 2], 000);
    }

    // color 3
    p.mem.storeb(0x0008, 0xFF);
    p.render_scanline(0);
    for i in 0..8 {
        assert_eq!(p.pixeldata[i*3 + 0], 188);
        assert_eq!(p.pixeldata[i*3 + 1], 040);
        assert_eq!(p.pixeldata[i*3 + 2], 068);
    }

    // color 2
    p.mem.storeb(0x0000, 0x00);
    p.render_scanline(0);
    for i in 0..8 {
        assert_eq!(p.pixeldata[i*3 + 0], 188);
        assert_eq!(p.pixeldata[i*3 + 1], 000);
        assert_eq!(p.pixeldata[i*3 + 2], 000);
    }

    // multiple colors in one row of a tile
    p.mem.storeb(0x0000, 0x0F);
    p.render_scanline(0);
    for i in 0..4 {
        assert_eq!(p.pixeldata[i*3 + 0], 188);
        assert_eq!(p.pixeldata[i*3 + 1], 000);
        assert_eq!(p.pixeldata[i*3 + 2], 000);
    }
    for i in 4..8 {
        assert_eq!(p.pixeldata[i*3 + 0], 188);
        assert_eq!(p.pixeldata[i*3 + 1], 040);
        assert_eq!(p.pixeldata[i*3 + 2], 068);
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
