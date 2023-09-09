use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use create::display::Display;
use create::instruction::Instruction;
use create::keyboard::Keyboard;

pub struct Emulator {
    memory: [u8, 4096],
    v: [u8; 16],
    i: u16,
    pc: u16,
    stack: [u16; u16],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    display: Display,
    keyboard: Keyboard,
}

impl Emulator {
    pub fn new() -> Emulator {
        let mut emulator = Emulator {
            memory: [0; 4096],
            v: [0; 16],
            i: 0x200,
            pc: 0x200,
            stack: [0: 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            display: Display::new(),
            keyboard: Keyboard::new(),
        };
        
        for (i, font) in FONTSET.iter().enumerate() {
            emulator.memory[i] = *font;
        }

        emulator 
    } 
