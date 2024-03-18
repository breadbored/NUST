# NUST

[WIP] A cycle accurate NES emulator in Rust, with minimal dependencies

## Mission

Create a portable, cycle accurate NES emulator in Rust that requires minimal porting to other systems.

Drawing and GUI will need to be system specific regardless of implementation.

~~Multithreading is kept barebones for the purpose of easily swapping. While this technically can be switched to an async event system rather than multithreading with 4 lines of code changed, performance will take a major hit because of the thread sleep logic that each thread uses. Please be aware that single threaded systems should refactor the timing mechanism found in the CPU instructions and the `main.rs` thread spawner.~~

Multithreading has been removed as it is uncessary with the current timing method. Multithreading the simple architecture provides 0 benefits to powerful systsems and is a hinderance to low power systems with only 1-4 threads. I may revisit adding a compiler flag to enable multithreading in the future.

## TODO

- [x] Implement cartridges
  - [x] Read and parse iNES and NES 2.0 ROM files
  - [ ] Implement all memory bank controllers
    - [ ] including unauthorized 3rd party mappers
  - [ ] Implement expansions (see memory mapping)
  - [ ] Bank switching (see memory mapping)
- [x] Implement memory mapping
  - [x] ROM
  - [x] RAM (and mirroring)
  - [ ] APU
  - [ ] PPU
  - [ ] Controller
  - [ ] Cartridge expansions (see cartridges)
  - [ ] SRAM
  - [ ] RTC
  - [ ] Bank switching (see cartridges)
- [ ] CPU
  - [x] Implement all registers
    - [x] Accumulator
    - [x] X
    - [x] Y
    - [x] Program Counter
    - [x] Stack Pointer
    - [x] Status
  - [ ] Implement all [6502 assembly instructions](https://www.masswerk.at/6502/6502_instruction_set.html) and their variants, including illegal opcodes (in progress, 61 / 255)
    - [x] ADC
    - [x] AND
    - [x] ASL
    - [x] BCC
    - [ ] BCS
    - [ ] BEQ
    - [ ] BIT
    - [ ] BMI
    - [x] BNE
    - [ ] BPL
    - [ ] BRK
    - [ ] BVC
    - [ ] BVS
    - [ ] CLC
    - [ ] CLD
    - [ ] CLI
    - [ ] CLV
    - [ ] CMP
    - [ ] CPX
    - [ ] CPY
    - [x] DEC
    - [x] DEX
    - [x] DEY
    - [ ] EOR
    - [ ] INC
    - [ ] INX
    - [ ] INY
    - [x] JMP
    - [x] JSR
    - [x] LDA
    - [x] LDX
    - [x] LDY
    - [ ] LSR
    - [x] NOP
    - [x] ORA
    - [ ] PHA
    - [ ] PHP
    - [ ] PLA
    - [ ] PLP
    - [ ] ROL
    - [ ] ROR
    - [ ] RTI
    - [ ] RTS
    - [ ] SBC
    - [ ] SEC
    - [ ] SED
    - [ ] SEI
    - [x] STA
    - [x] STX
    - [x] STY
    - [ ] TAX
    - [ ] TAY
    - [ ] TSX
    - [ ] TXA
    - [ ] TXS
    - [ ] TYA
    - [ ] ... more than I want to type
  - [ ] Implement vram reading/writing and blocking while vram is in use by PPU
- [ ] PPU
  - [ ] Implement registers
  - [x] Implement on a seperate thread with it's own timing and clock cycles
  - [ ] Implement vram locking and CPU blocking reads/writes
  - [ ] Parse CHR ROM and palettes for tile drawing
  - [ ] Tile drawing
    - [ ] Background
    - [ ] Foreground
- [ ] APU
  - [ ] Implement registers
  - [ ] Pulse Channel
  - [ ] Triangle Channel
  - [ ] Noise Channel
  - [ ] DMC
  - [ ] Status
  - [ ] Frame Counter
  - [ ] Mixer
- [ ] System graphics (\* = most experience, likely the first targets)
  - [ ] OpenGL (latest, all)\*
  - [ ] OpenGL (v1.1, for the N64 via libdragon)\*
  - [ ] Metal (Mac, iOS)\*
  - [ ] Vulkan (Windows, Android, Linux)\*
  - [ ] Deku (Nintendo Switch)
  - [ ] WebGL (webassembly target)
  - [ ] HTML Canvas (webassembly target because it would be funny)
