use std::fmt;

mod opcode;

const FONT_SET_SIZE: usize = 80;

pub struct Cpu {
    memory: [u8; 4096],
    registers: [u8; 16],

    index: u16,
    pc: u16,

    stack: [u16; 16],
    sp: u16
}

impl Cpu {
    pub fn initialize() -> Cpu {
        return Cpu {
            memory: [0; 4096],
            registers: [0; 16],
            index: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0
        }
    }

    pub fn load_font_set(&mut self, font_set: &[u8; FONT_SET_SIZE]) {
        for index in 0..FONT_SET_SIZE {
            self.memory[index] = font_set[index];
        }
    }

    pub fn cycle(&mut self) {
        let inst: opcode::Instruction = opcode::Instruction(
           ((self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize + 1] as u16) as u16
        );
        let op: opcode::Opcode = opcode::Instruction::decode_instruction(&inst);
        self.run_opcode(op, inst);
    }

    fn run_opcode(&mut self, op: opcode::Opcode, instruction: opcode::Instruction) {
        use opcode::Opcode;
        let opcode::Instruction(inst) = instruction;
        let vx: usize = (inst & 0x0F00) >> 8;
        let vy: usize = (inst & 0x00F0) >> 4;
        let nn: u8 = (inst & 0xFF) as u8;
        let nnn: u16 = inst & 0xFFF;
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
            Opcode::SNE => {
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
            },
            Opcode::ADDN => {
                self.registers[vx] += nn;
            },
            Opcode::LDY => {
                self.registers[vx] = self.registers[vy];
            },
            Opcode::OR => {
                self.registers[vx] |= self.registers[vy];
            },
            Opcode::AND => {
                self.registers[vx] &= self.registers[vy];
            },
            Opcode::XOR => {
                self.registers[vx] ^= self.registers[vy];
            },
            Opcode::ADDY => {
                self.registers[vx] += self.registers[vy];
            },
            Opcode::SUB => {
                self.registers[vx] -= self.registers[vy];
            },
            Opcode::SHR => {
                self.registers[vx] = self.registers[vx] >> 1;
            },
            Opcode::SUBN => {
                self.registers[vx] = self.registers[vy] - self.registers[vx];
            },
            Opcode::SHL => {
                self.registers[vx] = self.registers[vx] << 1;
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
            },
            Opcode::JPV => {
                self.pc = nnn + self.registers[0] as u16;
            },
            Opcode::RND => {
                //TODO: implement random
            },
            _ => panic!("Opcode {:?} not implemented", op)
        }
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..80 {
            write!(f, "{:#X}, ", self.memory[i]);
        };

        write!(f, "")
    }
}
