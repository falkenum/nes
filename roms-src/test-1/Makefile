# rm temp/palette.o
# rm temp/palette.nes

ROM_NAME=test-1
# OUT_DIR=
# ROM_FILE=$(ROM_NAME).nes
# OBJ_FILE=$(ROM_NAME).o

all: $(ROM_NAME).s nrom.cfg
	ca65 $(ROM_NAME).s -g -o build/$(ROM_NAME).o
	ld65 -C nrom.cfg -o build/$(ROM_NAME).nes build/$(ROM_NAME).o
run: all
	higan build/$(ROM_NAME).nes
