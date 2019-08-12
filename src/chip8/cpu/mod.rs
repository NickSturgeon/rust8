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
        match op {
            Opcode::JP => {
                self.pc = inst & 0xFFF;
            },
            Opcode::CALL => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = inst & 0xFFF;
            },
            Opcode::SEN => {
                let vx = (inst & 0x0F00) >> 8;
                if self.registers[vx as usize] == (inst & 0xFF) as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
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
