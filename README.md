# Intro
This is a work-in-progress emulator for the Nintendo Entertainment System, written in Rust. This project primarily exists as a learning experience for me and I do not know if/when it will be complete.
# Project Status
The emulator currently runs Donkey Kong without audio. In theory, it should also run any other game that doesn't require any of the items/features in the Project TODO section. So far, I've only tested Donkey Kong.
# Project TODO
  * implement sprite zero hit (and sprite overflow? is that used by any game?)
  * fix individual scanline rendering (currently accurate in rendering whole frames at once, not in parallel with CPU)
  * implement PPU mask emphasis/grayscale
  * implement the APU (audio)
  * implement scrolling
  * implement second controller
  * implement horizontal mirroring
  * implement more cartridge mappers besides mapper 0 (known as NROM, used by Donkey Kong and Super Mario Bros)
# Dependencies
This should build and run on any system that has Rust, Cargo, and SDL2 installed (Windows, MacOS, or Linux). However, I have only built and tested this on a 64 bit Pop!_OS 18.04 Linux Machine. For me, installing dependencies would look like:`sudo apt install git rustc cargo libsdl2-2.0-0 libsdl2-dev`.
# Installation
Once dependencies are installed, building the project is as simple as cloning, changing to the project directory, and using Cargo. On Linux, this looks like:`git clone https://github.com/falkenum/nes.git && cd nes && cargo build --release`. The executable will be `target/release/nes`.
# Running and Controls
Before running, you need a ROM to run. This type of file has the `.nes` extension. You can find ROMs online pretty easily. From the root project directory, once the project is built, run the emulator with `target/release/nes /path/to/rom.nes`.

Controls are currently hard-coded as follows: 

NES button | Key
---------- | ---
A | a
B | s
Select | z
Start | x
Up | Up arrow key
Down | Down arrow key
Left | Left arrow key
Right | Right arrow key
