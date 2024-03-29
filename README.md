# CARESemu: Cycle Accurate Rust Entertainment System Emulator

A [WIP] cycle accurate Nintendo Entertainment System (NES) emulator in Rust, with minimal dependencies

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/breadcodes)

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
  - [ ] Implement all [6502 assembly instructions](https://www.masswerk.at/6502/6502_instruction_set.html) and their variants, including illegal opcodes (in progress)
    - [x] ADC (Add with Carry) [Immediate, Zero Page, Zero Page X, Absolute, Absolute X, Absolute Y, Indirect X, Indirect Y]
    - [x] AND (Logical AND) [Immediate, Zero Page, Zero Page X, Absolute, Absolute X, Absolute Y, Indirect X, Indirect Y]
    - [x] ASL (Arithmetic Shift Left) [Accumulator, Zero Page, Zero Page X, Absolute, Absolute X]
    - [x] BCC (Branch if Carry Clear) [Relative]
    - [x] BCS (Branch if Carry Set) [Relative]
    - [x] BEQ (Branch if Equal) [Relative]
    - [x] BIT (Bit Test) [Zero Page, Absolute]
    - [x] BMI (Branch if Minus) [Relative]
    - [x] BNE (Branch if Not Equal) [Relative]
    - [x] BPL (Branch if Positive) [Relative]
    - [x] BRK (Force Interrupt) [Implied]
    - [x] BVC (Branch if Overflow Clear) [Relative]
    - [x] BVS (Branch if Overflow Set) [Relative]
    - [x] CLC (Clear Carry Flag) [Implied]
    - [x] CLD (Clear Decimal Mode) [Implied]
    - [x] CLI (Clear Interrupt Disable) [Implied]
    - [x] CLV (Clear Overflow Flag) [Implied]
    - [x] CMP (Compare) [Immediate, Zero Page, Zero Page X, Absolute, Absolute X, Absolute Y, Indirect X, Indirect Y]
    - [x] CPX (Compare X Register) [Immediate, Zero Page, Zero Page X, Absolute]
    - [x] CPY (Compare Y Register) [Immediate, Zero Page, Zero Page X, Absolute]
    - [x] DEC (Decrement Memory) [Zero Page, Zero Page X, Absolute, Absolute X]
    - [x] DEX (Decrement X Register) [Implied]
    - [x] DEY (Decrement Y Register) [Implied]
    - [ ] EOR (Exclusive OR) [Immediate, Zero Page, Zero Page X, Absolute, Absolute X, Absolute Y, Indirect X, Indirect Y]
    - [ ] INC (Increment Memory) [Zero Page, Zero Page X, Absolute, Absolute X]
    - [ ] INX (Increment X Register) [Implied]
    - [ ] INY (Increment Y Register) [Implied]
    - [x] JMP (Jump) [Absolute, Indirect]
    - [x] JSR (Jump to Subroutine) [Absolute]
    - [x] LDA (Load Accumulator) [Immediate, Zero Page, Zero Page X, Absolute, Absolute X, Absolute Y, Indirect X, Indirect Y]
    - [x] LDX (Load X Register) [Immediate, Zero Page, Zero Page Y, Absolute, Absolute Y]
    - [x] LDY (Load Y Register) [Immediate, Zero Page, Zero Page X, Absolute, Absolute X]
    - [x] LSR (Logical Shift Right) [Accumulator, Zero Page, Zero Page X, Absolute, Absolute X]
    - [x] NOP (No Operation) [Implied]
    - [x] ORA (Logical Inclusive OR) [Immediate, Zero Page, Zero Page X, Absolute, Absolute X, Absolute Y, Indirect X, Indirect Y]
    - [x] PHA (Push Accumulator) [Implied]
    - [x] PHP (Push Processor Status) [Implied]
    - [x] PLA (Pull Accumulator) [Implied]
    - [x] PLP (Pull Processor Status) [Implied]
    - [x] ROL (Rotate Left) [Accumulator, Zero Page, Zero Page X, Absolute, Absolute X]
    - [x] ROR (Rotate Right) [Accumulator, Zero Page, Zero Page X, Absolute, Absolute X]
    - [x] RTI (Return from Interrupt) [Implied]
    - [x] RTS (Return from Subroutine) [Implied]
    - [x] SBC (Subtract with Carry) [Immediate, Zero Page, Zero Page X, Absolute, Absolute X, Absolute Y, Indirect X, Indirect Y]
    - [x] SEC (Set Carry Flag) [Implied]
    - [x] SED (Set Decimal Flag) [Implied]
    - [x] SEI (Set Interrupt Disable) [Implied]
    - [x] STA (Store Accumulator) [Zero Page, Zero Page X, Absolute, Absolute X, Absolute Y, Indirect X, Indirect Y]
    - [x] STX (Store X Register) [Zero Page, Zero Page Y, Absolute]
    - [x] STY (Store Y Register) [Zero Page, Zero Page X, Absolute]
    - [x] TAX (Transfer Accumulator to X) [Implied]
    - [x] TAY (Transfer Accumulator to Y) [Implied]
    - [x] TSX (Transfer Stack Pointer to X) [Implied]
    - [x] TXA (Transfer X to Accumulator) [Implied]
    - [x] TXS (Transfer X to Stack Pointer) [Implied]
    - [x] TYA (Transfer Y to Accumulator) [Implied]
    - [ ] Illegal Opcodes
      - [ ] AHX
      - [ ] ALR
      - [ ] ANC
      - [ ] ARR
      - [ ] AXS
      - [ ] DCP
      - [ ] ISC
      - [ ] KIL
      - [ ] LAS
      - [ ] LAX
      - [ ] RLA
      - [ ] RRA
      - [ ] SAX
      - [ ] SHX
      - [ ] SHY
      - [ ] SLO
      - [ ] SRE
      - [ ] TAS
      - [ ] XAA
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
