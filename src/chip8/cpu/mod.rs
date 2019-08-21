use std::fmt;
use std::io;
use std::io::prelude::*;
use std::fs::File;

extern crate sdl2;
extern crate rand;

mod opcode;
mod input;
mod graphics;

const FONT_SET_SIZE: usize = 80;
const PROGRAM_START: usize = 0x200;

pub struct Cpu {
    memory: [u8; 4096],
    registers: [u8; 16],

    index: u16,
    pc: u16,

    stack: [u16; 16],
    sp: u16,

    graphics: sdl2::render::Canvas<sdl2::video::Window>,
    input: sdl2::EventPump
}

impl Cpu {
    pub fn initialize() -> Cpu {
        let sdl_context = sdl2::init().unwrap();

        return Cpu {
            memory: [0; 4096],
            registers: [0; 16],
            index: 0,
            pc: PROGRAM_START as u16,
            stack: [0; 16],
            sp: 0,
            graphics: graphics::initialize(&sdl_context),
            input: input::initialize(&sdl_context)
        }
    }

    pub fn load_font_set(&mut self) {
        for index in 0..FONT_SET_SIZE {
            self.memory[index] = graphics::FONT_SET[index];
        }
    }

    pub fn load_game(&mut self, game: &String) {
       let mut file = File::open(game).unwrap();
       let mut buffer = Vec::new();

        for i in 0..file.read_to_end(&mut buffer).unwrap() {
            self.memory[PROGRAM_START + i] = buffer[i];
        }
    }

    pub fn cycle(&mut self) {
        let inst: opcode::Instruction = opcode::Instruction(
           ((self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize + 1] as u16) as u16
        );
        let op: opcode::Opcode = opcode::Instruction::decode_instruction(&inst);
        self.run_opcode(op, inst);
        input::poll_for_event(&mut self.input);
        graphics::draw(&mut self.graphics);
    }

    fn run_opcode(&mut self, op: opcode::Opcode, instruction: opcode::Instruction) {
        use opcode::Opcode;
        let opcode::Instruction(inst) = instruction;
        let vx: usize = ((inst & 0x0F00) >> 8) as usize;
        let vy: usize = ((inst & 0x00F0) >> 4) as usize;
        let nn: u8 = (inst & 0xFF) as u8;
        let nnn: u16 = inst & 0xFFF;
        println!("opcode: {:?}", op);
        match op {
            Opcode::JP => {
                self.pc = nnn;
            },
            Opcode::CALL => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            },
            Opcode::SEN => {
                if self.registers[vx] == nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            Opcode::SNEN => {
                if self.registers[vx] != nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            Opcode::SEY => {
                if self.registers[vx] == self.registers[vy] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            Opcode::LDN => {
                self.registers[vx] = nn;
                self.pc += 2;
            },
            Opcode::ADDN => {
                self.registers[vx] += nn;
                self.pc += 2;
            },
            Opcode::LDY => {
                self.registers[vx] = self.registers[vy];
                self.pc += 2;
            },
            Opcode::OR => {
                self.registers[vx] |= self.registers[vy];
                self.pc += 2;
            },
            Opcode::AND => {
                self.registers[vx] &= self.registers[vy];
                self.pc += 2;
            },
            Opcode::XOR => {
                self.registers[vx] ^= self.registers[vy];
                self.pc += 2;
            },
            Opcode::ADDY => {
                self.registers[vx] += self.registers[vy];
                self.pc += 2;
            },
            Opcode::SUB => {
                self.registers[vx] -= self.registers[vy];
                self.pc += 2;
            },
            Opcode::SHR => {
                self.registers[vx] = self.registers[vx] >> 1;
                self.pc += 2;
            },
            Opcode::SUBN => {
                self.registers[vx] = self.registers[vy] - self.registers[vx];
                self.pc += 2;
            },
            Opcode::SHL => {
                self.registers[vx] = self.registers[vx] << 1;
                self.pc += 2;
            },
            Opcode::SNEY => {
                if self.registers[vx] != self.registers[vy] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            Opcode::LDI => {
                self.index = nnn;
                self.pc += 2;
            },
            Opcode::JPV => {
                self.pc = nnn + self.registers[0] as u16;
            },
            Opcode::RND => {
                self.registers[vx] = nn & rand::random();
                self.pc += 2;
            },
            _ => panic!("Opcode {:?} not implemented", op)
        }
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //for i in 0x200..0x210 {
        //    write!(f, "mem[{}]: {:#X}, ", i, self.memory[i]);
        //};
        writeln!(f, "pc:      {:#06X}", self.pc);
        writeln!(f, "i:       {:#06X}", self.index);
        for i in 0..16 {
            writeln!(f, "reg[{:02}]: {:#04X}", i, self.registers[i]);
        };



        write!(f, "")
    }
}
