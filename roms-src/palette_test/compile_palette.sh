#!/bin/bash

# rm temp/palette.o
# rm temp/palette.nes
ca65 palette.s -g -o temp/palette.o
ld65 -C nrom.cfg -o ../../palette.nes temp/palette.o
# python3 palette_symbols.py
