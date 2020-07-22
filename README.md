# Yet Another Chip-8 Emulator.

# Goals

Firstly, the goal was to implement a Chip-8 emulator. But there was a couple things I wanted to take a look at in rust regarding project structure.

1. Rust recommends "small crates"
2. Use traits to see what the dependency injection story is like in Rust when you want to build up a largish code base.
3. cargo allows for workspaces for multi crate projects.

## Structure

Each crate builds an important part of the emulator.

1. **Model** defines all the behavior of various components through traits.
1. **Instruction** Defines the instruction parsing.
1. **Data** is a bunch of wrapper objects around `u8` and `u16`.
1. **Vm** is hte main machine that depends oneverything.
1. **Emulator** is an example that constructs a `vm` from all the parts. It provides the interpretation layer between the host and the vm by mapping the framebuffer output, and key input.
1. **RomLibrary** is a library of games that can be played.
1. Everything else is a component that the vm requires to be able to run. 

## Todo

- Build a web emulator frontend for the vm.
- Host a blob database for storing the roms.
