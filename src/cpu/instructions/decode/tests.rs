    use ::{ Memory, CPU, Cartridge };

    #[test]
    fn cycles() {
        let mut c = CPU::test();
        assert_eq!(c.cycle_count, 0);

        // simple instruction
        c.mem.storeb(0x8000, 0xA9);
        c.mem.storeb(0x8001, 0x55);
        c.tick();
        assert_eq!(c.cycle_count, 1);
        c.step();
        assert_eq!(c.cycle_count, 0);

        // no page crossing on lda indirect, y: 5 cycles
        c.mem.storeb(0x00FD, 0xBB);
        c.mem.storeb(0x00F0, 0x0D);
        c.y = 0xF0;
        c.mem.storeb(0x8002, 0xB1);
        c.mem.storeb(0x8003, 0xF0);
        c.tick();
        assert_eq!(c.cycle_count, 4);
        c.step();
        assert_eq!(c.a, 0xBB);

        // page crossing on lda indirect, y: 5+1 cycles
        c.mem.storeb(0x00F0, 0xFF);
        c.mem.storeb(0x00F1, 0x01);
        c.mem.storeb(0x0200, 0x07);
        c.y = 0x01;
        c.mem.storeb(0x8004, 0xB1);
        c.mem.storeb(0x8005, 0xF0);
        c.tick();
        assert_eq!(c.cycle_count, 5);
        c.step();
        assert_eq!(c.a, 0x07);

        // page crossing on lda absolute, x: 4+1 cycles
        c.mem.storeb(0x0200, 0x08);
        c.x = 0x01;
        c.mem.storeb(0x8006, 0xBD);
        c.mem.storeb(0x8007, 0xFF);
        c.mem.storeb(0x8008, 0x01);
        c.tick();
        assert_eq!(c.cycle_count, 4);
        c.step();
        assert_eq!(c.a, 0x08);

        // page crossing on lda absolute, y: 4+1 cycles
        c.mem.storeb(0x0200, 0x09);
        c.y = 0x01;
        c.mem.storeb(0x8009, 0xB9);
        c.mem.storeb(0x800A, 0xFF);
        c.mem.storeb(0x800B, 0x01);
        c.tick();
        assert_eq!(c.cycle_count, 4);
        c.step();
        assert_eq!(c.a, 0x09);
    }

    #[test]
    fn addr_modes() {

        let mut c = CPU::test();

        c.x = 0x00;
        assert_eq!(c.absolute_x(0x0000_u16), 0x0000_u16);
        assert_eq!(c.absolute_x(0xFFFF_u16), 0xFFFF_u16);
        c.x = 0xFF;
        assert_eq!(c.absolute_x(0x0000_u16), 0x00FF_u16);
        assert_eq!(c.absolute_x(0xFFFF_u16), 0x00FE_u16);

        c.y = 0x00;
        assert_eq!(c.absolute_y(0x0000_u16), 0x0000_u16);
        assert_eq!(c.absolute_y(0xFFFF_u16), 0xFFFF_u16);
        c.y = 0xFF;
        assert_eq!(c.absolute_y(0x0000_u16), 0x00FF_u16);
        assert_eq!(c.absolute_y(0xFFFF_u16), 0x00FE_u16);

        assert_eq!(c.zero_page(0x00_u8), 0x0000_u16);
        assert_eq!(c.zero_page(0xFF_u8), 0x00FF_u16);

        c.x = 0x00;
        assert_eq!(c.zero_page_x(0x00_u8), 0x0000_u16);
        assert_eq!(c.zero_page_x(0xFF_u8), 0x00FF_u16);
        c.x = 0xFF;
        assert_eq!(c.zero_page_x(0x00_u8), 0x00FF_u16);
        // this is an interesting case: zero page addressing with index
        // specifies (according to MOS datasheet) essentially that
        // the index is added to the argument before it is extended to
        // 16 bits, so any carry from that addition is dropped
        assert_eq!(c.zero_page_x(0xFF_u8), 0x00FE_u16);

        c.y = 0x00;
        assert_eq!(c.zero_page_y(0x00_u8), 0x0000_u16);
        assert_eq!(c.zero_page_y(0xFF_u8), 0x00FF_u16);
        c.y = 0xFF;
        assert_eq!(c.zero_page_y(0x00_u8), 0x00FF_u16);
        assert_eq!(c.zero_page_y(0xFF_u8), 0x00FE_u16);

        c.mem.storeb(0x0, 0xA);
        c.mem.storeb(0x1, 0xB);
        c.mem.storeb(0x2, 3);
        c.mem.storeb(0xFE, 1);
        c.mem.storeb(0xFF, 2);
        c.mem.storeb(0x100, 4);
        c.mem.storeb(0x1FF, 5);
        c.mem.storeb(0x200, 6);

        assert_eq!(c.indirect(0x0000_u16), 0x0B0A_u16);
        assert_eq!(c.indirect(0x00FE_u16), 0x0201_u16);

        // wrap around
        assert_eq!(c.indirect(0x00FF_u16), 0x0A02_u16);
        // wrap around
        assert_eq!(c.indirect(0x01FF_u16), 0x0405_u16);

        c.x = 0x00;
        assert_eq!(c.indirect_x(0x00_u8), 0x0B0A_u16);
        assert_eq!(c.indirect_x(0xFE_u8), 0x0201_u16);
        c.x = 0xFE;
        // wrap around
        assert_eq!(c.indirect_x(0x02_u8), 0x0B0A_u16);
        assert_eq!(c.indirect_x(0x00_u8), 0x0201_u16);

        // wrap around
        assert_eq!(c.indirect_x(0x01_u8), 0x0A02_u16);

        c.y = 0x00;
        assert_eq!(c.indirect_y(0x00_u8), 0x0B0A_u16);
        // wrap around
        assert_eq!(c.indirect_y(0xFF_u8), 0x0A02_u16);

        // All kinds of wrap around that I'm still confused about
        c.y = 0x01;
        assert_eq!(c.indirect_y(0xFF_u8), 0x0A03_u16);

        c.y = 0xFE;
        assert_eq!(c.indirect_y(0xFF_u8), 0x0B00_u16);

        c.pc = 0x8000;
        assert_eq!(c.relative(0x50), 0x8050);
        assert_eq!(c.relative(0xFF), 0x7FFF);
        c.pc = 0x8080;
        assert_eq!(c.relative(0x80), 0x8000);
    }
