use std::{fs::{self}, u16};

const KB : u16 = 1024;
const START_ADDRESS : u16 = 0x200;
const ADDRESS_MASK : u16 = 0x0FFF;
const BYTE_CONSTANT_MASK : u16 = 0x00FF;
const HALF_BYTE_CONSTANT_MASK : u16 = 0x000F;
const X_REGISTER_MASK : u16 = 0x0F00;
const Y_REGISTER_MASK : u16 = 0x00F0;
struct Chip8System {
    memory : [u8 ; 4 * KB as usize],
    display : [bool ; 64 * 32 as usize],
    index_register : u16,
    pc : u16,
    delay_timer : u8,
    sound_timer : u8,
    registers : [u8 ; 16],
    stack : Vec<u16>
}

impl Chip8System {
    pub fn new(mem : [u8; 4096]) -> Self {
        Self {
            memory : mem,
            display : [false ; 64 * 32 as usize],
            index_register : 0,
            pc : START_ADDRESS,
            delay_timer : 0,
            sound_timer : 0,
            registers : [0; 16],
            stack : Vec::new()
        }
    }
}

pub struct Interpreter {
    chip8 : Chip8System,
}

impl Interpreter {
    pub fn new(filepath : &str) -> Self {
        let buffer = fs::read(filepath).unwrap();
        let mut mem: [u8; 4096] = [0 ; 4096];

        for (i, byte) in buffer.iter().enumerate() {
            mem[0x200 + i] = *byte;
        }

        Self {
            chip8 : Chip8System::new(mem),
        }
    }

    pub fn next_instruction(&mut self){

        let opcode: u16 = ((self.chip8.memory[self.chip8.pc as usize] as u16) << 8) // High byte
                            | (self.chip8.memory[self.chip8.pc as usize + 1] as u16); // Low byte

        let NNN: u16 = opcode & ADDRESS_MASK;
        let NN: u8 = (opcode & BYTE_CONSTANT_MASK) as u8;
        let N: u8 = (opcode & HALF_BYTE_CONSTANT_MASK) as u8;
        let register_x_index: usize = ((opcode & X_REGISTER_MASK) >> 8) as usize;
        let register_y_index: usize = ((opcode & Y_REGISTER_MASK) >> 4) as usize;

        // Decode instruction
        match opcode & 0xF000 {
            0x0000 => {
                if opcode & 0x00E0 != 0 {
                    self.clear_display();
                }
                if opcode & 0x00EE != 0 {
                    if let Some(addr) =  self.chip8.stack.pop() {
                        self.chip8.pc = addr; 
                    } else {
                        panic!("Stack underflow on instruction 0x00EE at PC = {}", self.chip8.pc);
                    }
                }
            },

            0x1000 => {
                self.chip8.pc = NNN;
            },

            0x2000 => {
                self.chip8.stack.push(self.chip8.pc);
                self.chip8.pc = NNN;
            },

            0x3000 => {
                if self.chip8.registers[register_x_index] == NN {
                    self.chip8.pc += 2;
                }
            },

            0x4000 => {
                if self.chip8.registers[register_x_index] != NN {
                    self.chip8.pc += 2;
                }
            },

            0x5000 => {
                if self.chip8.registers[register_x_index] == self.chip8.registers[register_y_index] {
                    self.chip8.pc += 2;
                }
            },

            0x6000 => {
                self.chip8.registers[register_x_index] = NN;
            },

            0x7000 => {
                self.chip8.registers[register_x_index] += NN;
            },

            0x8000 => {
            }


            _ => {}
        }
    }

    fn clear_display(&mut self) {
        self.chip8.display.fill(false);
    }
}