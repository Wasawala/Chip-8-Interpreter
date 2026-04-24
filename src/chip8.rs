use rand::RngExt;
use sdl3::{
    keyboard::{KeyboardState, Scancode},
    pixels::PixelFormat,
    render::Canvas,
    video::Window,
};
use std::fs::{self};

const KB: u16 = 1024;
const START_ADDRESS: u16 = 0x200;
const ADDRESS_MASK: u16 = 0x0FFF;
const BYTE_CONSTANT_MASK: u16 = 0x00FF;
const HALF_BYTE_CONSTANT_MASK: u16 = 0x000F;
const X_REGISTER_MASK: u16 = 0x0F00;
const Y_REGISTER_MASK: u16 = 0x00F0;
const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const FONT_START_ADDRESS: usize = 0x050;

struct Chip8System {
    memory: [u8; 4 * KB as usize],
    display: [bool; 64 * 32],
    address_register: u16,
    pc: u16,
    delay_timer: u8,
    sound_timer: u8,
    V: [u8; 16],
    stack: Vec<u16>,
}

impl Chip8System {
    pub fn new(mem: [u8; 4096]) -> Self {
        Self {
            memory: mem,
            display: [false; 64 * 32],
            address_register: 0,
            pc: START_ADDRESS,
            delay_timer: 0,
            sound_timer: 0,
            V: [0; 16],
            stack: Vec::new(),
        }
    }
}

pub struct Interpreter {
    chip8: Chip8System,
}

impl Interpreter {
    pub fn new(filepath: &str) -> Self {
        let buffer = fs::read(filepath).unwrap();
        let mut mem: [u8; 4096] = [0; 4096];

        for (i, byte) in buffer.iter().enumerate() {
            mem[0x200 + i] = *byte;
        }

        let font_values: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        mem[0x050..=0x09F].copy_from_slice(&font_values);

        Self {
            chip8: Chip8System::new(mem),
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormat::RGB24, WIDTH as u32, HEIGHT as u32)
            .unwrap();

        let mut data = [0u8; WIDTH * HEIGHT * 3];

        for i in 0..(WIDTH * HEIGHT) {
            let offset = i * 3;

            if self.chip8.display[i] {
                data[offset] = 255;
                data[offset + 1] = 255;
                data[offset + 2] = 255;
            } else {
                data[offset] = 0;
                data[offset + 1] = 0;
                data[offset + 2] = 0;
            }
        }

        texture.update(None, &data, WIDTH * 4).unwrap();
        texture.set_scale_mode(sdl3::render::ScaleMode::Nearest);
        canvas.copy(&texture, None, None).unwrap();
    }

    pub fn next_instruction(&mut self, keyboard_state: KeyboardState) {
        let opcode: u16 = ((self.chip8.memory[self.chip8.pc as usize] as u16) << 8) // High byte
                            | (self.chip8.memory[self.chip8.pc as usize + 1] as u16); // Low byte

        let NNN: u16 = opcode & ADDRESS_MASK;
        let NN: u8 = (opcode & BYTE_CONSTANT_MASK) as u8;
        let N: u8 = (opcode & HALF_BYTE_CONSTANT_MASK) as u8;
        let I = NNN;
        let X: usize = ((opcode & X_REGISTER_MASK) >> 8) as usize;
        let Y: usize = ((opcode & Y_REGISTER_MASK) >> 4) as usize;

        self.chip8.pc += 2;

        match opcode & 0xF000 {
            0x0000 => match opcode & 0x0FFF {
                0x00E0 => {
                    self.clear_display();
                }

                0x00EE => {
                    if let Some(addr) = self.chip8.stack.pop() {
                        self.chip8.pc = addr;
                    } else {
                        panic!(
                            "Stack underflow on instruction 0x00EE at PC = {}",
                            self.chip8.pc
                        );
                    }
                }

                _ => {
                    println!("unknown instruction {:04X}", opcode);
                }
            },

            // goto NNN
            0x1000 => {
                self.chip8.pc = NNN;
            }

            // call *(0xNNN)
            0x2000 => {
                self.chip8.stack.push(self.chip8.pc);
                self.chip8.pc = NNN;
            }

            // skip if Vx == NN
            0x3000 => {
                if self.chip8.V[X] == NN {
                    self.chip8.pc += 2;
                }
            }

            // skip if Vx != NN
            0x4000 => {
                if self.chip8.V[X] != NN {
                    self.chip8.pc += 2;
                }
            }

            // skip if Vx == Vy
            0x5000 => {
                if opcode & 0x000F == 0 && self.chip8.V[X] == self.chip8.V[Y] {
                    self.chip8.pc += 2;
                }
            }

            // Vx = NN
            0x6000 => {
                self.chip8.V[X] = NN;
            }

            // Vx += NN
            0x7000 => {
                self.chip8.V[X] += NN;
            }

            0x8000 => match opcode & 0xF {
                // Vx = Vy
                0x0 => {
                    self.chip8.V[X] = self.chip8.V[Y];
                }

                // Vx |= Vy
                0x1 => {
                    self.chip8.V[X] |= self.chip8.V[Y];
                }

                // Vx &= Vy
                0x2 => {
                    self.chip8.V[X] &= self.chip8.V[Y];
                }

                // Vx ^= Vy
                0x3 => {
                    self.chip8.V[X] ^= self.chip8.V[Y];
                }

                // Vx += Vy, VF = overflow
                0x4 => {
                    let sum = self.chip8.V[X].wrapping_add(self.chip8.V[Y]);

                    self.chip8.V[0xF] = if sum < self.chip8.V[X] { 1 } else { 0 };
                    self.chip8.V[X] = sum;
                }

                // Vx -= Vy, VF = 0 if underflow
                0x5 => {
                    let dif = self.chip8.V[X].wrapping_sub(self.chip8.V[Y]);

                    self.chip8.V[0xF] = if self.chip8.V[Y] > self.chip8.V[X] {
                        0
                    } else {
                        1
                    };
                    self.chip8.V[X] = dif;
                }

                // VX >>= 1
                0x6 => {
                    let ls_bit = self.chip8.V[X] & 0b1;
                    self.chip8.V[0xF] = ls_bit;
                    self.chip8.V[X] >>= 1;
                }

                // Vx = Vy - Vx
                0x7 => {
                    let dif = self.chip8.V[Y].wrapping_sub(self.chip8.V[X]);
                    self.chip8.V[0xF] = if self.chip8.V[X] > self.chip8.V[Y] {
                        0
                    } else {
                        1
                    };
                    self.chip8.V[X] = dif;
                }

                // Vx <<= 1
                0xE => {
                    self.chip8.V[0xF] = (self.chip8.V[X] >> 7) & 1;
                    self.chip8.V[X] <<= 1;
                }

                _ => {
                    println!("unknown instruction {:04X}", opcode);
                }
            },

            // skip if Vx != Vy
            0x9000 => {
                if opcode & 0x000F == 0 && self.chip8.V[X] != self.chip8.V[Y] {
                    self.chip8.pc += 2;
                }
            }

            // I = NNN
            0xA000 => {
                self.chip8.address_register = I;
            }

            // PC = V0 + NNN
            0xB000 => {
                self.chip8.pc = (self.chip8.V[0] as u16) + NNN;
            }

            // Vx = rand() & NN
            0xC000 => {
                let mut rng = rand::rng();
                self.chip8.V[X] = rng.random_range(0..=255) & NN;
            }

            // draw at (Vx, Vy)
            0xD000 => {
                self.draw_sprite(X as u8, Y as u8, N);
            }

            0xF000 => match opcode & 0xFF {
                // Vx = delay_timer
                0x07 => {
                    self.chip8.V[X] = self.chip8.delay_timer;
                }

                // Vx = key
                0x0A => {
                    let key = self.decode_keycode(keyboard_state);
                    if key <= 0xF {
                        self.chip8.V[X] = key;
                    } else {
                        self.chip8.pc -= 2;
                    }
                }

                // delay_timer = Vx
                0x15 => {
                    self.chip8.delay_timer = self.chip8.V[X];
                }

                // sound_timer = Vx
                0x18 => {
                    self.chip8.sound_timer = self.chip8.V[X];
                }

                // I += Vx
                0x1E => {
                    self.chip8.address_register += self.chip8.V[X] as u16;
                }

                // I = sprite at Vx
                0x29 => {
                    self.chip8.address_register =
                        (FONT_START_ADDRESS + (self.chip8.V[X] as usize * 5)) as u16;
                }

                // set BCD of Vx starting at I
                0x33 => {
                    self.chip8.memory[I as usize] = self.chip8.V[X] / 100;
                    self.chip8.memory[I as usize + 1] = (self.chip8.V[X] % 100) / 10;
                    self.chip8.memory[I as usize + 2] = self.chip8.V[X] % 10;
                }

                // store registers 0 to X starting at I
                0x55 => {
                    for i in 0..=X {
                        self.chip8.memory[I as usize + i] = self.chip8.V[i];
                    }
                }

                // load registers 0 to X starting at I
                0x65 => {
                    for i in 0..=X {
                        self.chip8.V[i] = self.chip8.memory[I as usize + i];
                    }
                }

                _ => {
                    println!("unknown instruction {:04X}", opcode);
                }
            },

            _ => {
                println!("unknown instruction {:04X}", opcode);
            }
        }
    }

    fn clear_display(&mut self) {
        self.chip8.display.fill(false);
    }

    fn draw_sprite(&mut self, x_index: u8, y_index: u8, N: u8) {
        let x_start: usize = (self.chip8.V[x_index as usize] as usize) % WIDTH;
        let y_start: usize = (self.chip8.V[y_index as usize] as usize) % HEIGHT;

        let mut collision: bool = false;

        'draw_y: for y in 0..N as usize {
            if (y_start + y) == HEIGHT {
                break 'draw_y;
            }

            let cur_byte = self.chip8.memory[(self.chip8.address_register + y as u16) as usize];

            'fill_x: for x in 0..8 {
                if (x_start + x) == WIDTH {
                    break 'fill_x;
                }
                let x_pos = (x_start + x) % WIDTH;
                let y_pos = (y_start + y) % HEIGHT;
                let cur_pos = (x_pos) + (y_pos) * WIDTH;

                let cur_pixel_state: bool = self.chip8.display[cur_pos];

                let next_pixel = (cur_byte >> (7 - x)) & 1;

                if cur_pixel_state && next_pixel == 1 {
                    collision = true;
                }

                self.chip8.display[cur_pos] ^= next_pixel == 1;
            }
        }

        self.chip8.V[0xF] = collision as u8;
    }

    pub fn dec_timers(&mut self) {
        if self.chip8.delay_timer > 0 {
            self.chip8.delay_timer -= 1;
        } else {
            self.chip8.delay_timer = 255;
        }

        if self.chip8.sound_timer > 0 {
            self.chip8.sound_timer -= 1;
        } else {
            self.chip8.sound_timer = 255;
        }
    }

    fn decode_keycode(&self, keyboard_state: KeyboardState) -> u8 {
        let mut matched: u8 = 0x10;

        for scancode in keyboard_state.scancodes() {
            if keyboard_state.is_scancode_pressed(scancode.0) {
                matched = match scancode.0 {
                    Scancode::_1 => 0x0,
                    Scancode::_2 => 0x1,
                    Scancode::_3 => 0x2,
                    Scancode::_4 => 0x3,
                    Scancode::Q => 0x4,
                    Scancode::W => 0x5,
                    Scancode::E => 0x6,
                    Scancode::R => 0x7,
                    Scancode::A => 0x8,
                    Scancode::S => 0x9,
                    Scancode::D => 0xA,
                    Scancode::F => 0xB,
                    Scancode::Z => 0xC,
                    Scancode::X => 0xD,
                    Scancode::C => 0xE,
                    Scancode::V => 0xF,

                    _ => 0x10,
                };
                if matched <= 0xF {
                    return matched;
                }
            }
        }

        matched
    }

    pub fn dump_memory(&self) {
        let mut i = 0;
        while i < self.chip8.memory.len() {
            if self.chip8.memory[i] == 0 && self.chip8.memory[i + 1] == 0 {
                i += 2;
            } else {
                println!(
                    "0x{:04X}, {:02X}{:02X}",
                    i,
                    self.chip8.memory[i],
                    self.chip8.memory[i + 1]
                );
                i += 2;
            }
        }
    }
}
