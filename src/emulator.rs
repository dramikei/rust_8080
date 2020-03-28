// use cpu::CPU;
use register::Reg;
use std;
pub mod cpu;
mod register;

pub struct Emulator {
    cycles8080: [u8;256],
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            cycles8080: [
                4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5, 7, 4, //0x00..0x0f
                4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5, 7, 4, //0x10..0x1f
                4, 10, 16, 5, 5, 5, 7, 4, 4, 10, 16, 5, 5, 5, 7, 4, //etc
                4, 10, 13, 5, 10, 10, 10, 4, 4, 10, 13, 5, 5, 5, 7, 4,
                
                5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5, //0x40..0x4f
                5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5,
                5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5,
                7, 7, 7, 7, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 7, 5,
                
                4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4, //0x80..8x4f
                4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,
                4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,
                4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,
                
                11, 10, 10, 10, 17, 11, 7, 11, 11, 10, 10, 10, 10, 17, 7, 11, //0xc0..0xcf
                11, 10, 10, 10, 17, 11, 7, 11, 11, 10, 10, 10, 10, 17, 7, 11, 
                11, 10, 10, 18, 17, 11, 7, 11, 11, 5, 10, 5, 17, 17, 7, 11, 
                11, 10, 10, 4, 17, 11, 7, 11, 11, 5, 10, 4, 17, 17, 7, 11
            ],
        }
    }


    pub fn load_rom(&self, cpu: &mut cpu::CPU, start: usize) {
        let x = std::include_bytes!("rom/invaders.rom");
        let mut i = 0;
        if x.len() > start+0xffff {
            panic!("PANIC: Rom size exceeds Memory!!");
        } else {
            while i< x.len() {
                cpu.memory[start+i] = x[i];
                i += 1;
            }
        }
    }
    fn extract_argument(opcode: u8) -> Reg {
        match opcode & 0x0F {
            0x0 | 0x8 => Reg::B,
            0x1 | 0x9 => Reg::C,
            0x2 | 0xA => Reg::D,
            0x3 | 0xB => Reg::E,
            0x4 | 0xC => Reg::H,
            0x5 | 0xD => Reg::L,
            0x6 | 0xE => Reg::HL,
            0x7 | 0xF => Reg::A,
            _ => panic!("Could not resolve operating register for opcode {:x}",opcode),
        }
    }

    fn write_to_reg(cpu: &mut cpu::CPU, to: Reg, value: u8) {
        match to {
            Reg::B => cpu.b = value,
            Reg::C => cpu.b = value,
            Reg::D => cpu.b = value,
            Reg::E => cpu.b = value,
            Reg::H => cpu.b = value,
            Reg::L => cpu.b = value,
            Reg::HL => {
                let addr = (((cpu.h as u16) << 8) | (cpu.l as u16)) as usize;
                cpu.memory[addr] = value;
            },
            Reg::A => cpu.b = value,
            _ => panic!("ADD CALLED FROM WRONG REG!")
        }
    }

    fn lxi(cpu: &mut cpu::CPU, to: Reg) {
        match to {
            Reg::B => {
                cpu.b = cpu.memory[(cpu.pc as usize) + 2];
                cpu.c = cpu.memory[(cpu.pc as usize) + 1];
            },
            Reg::D => {
                cpu.d = cpu.memory[(cpu.pc as usize) + 2];
                cpu.e = cpu.memory[(cpu.pc as usize) + 1];
            },
            Reg::H => {
                cpu.h = cpu.memory[(cpu.pc as usize) + 2];
                cpu.l = cpu.memory[(cpu.pc as usize) + 1];
            },
            Reg::SP => {
                cpu.sp = ((cpu.memory[(cpu.pc as usize) + 2] as u16) << 8 ) | (cpu.memory[(cpu.pc as usize) + 1] as u16);
            },
            _ => {
                panic!("LXI CALLED ON WRONG REG!");
            }
        }
        cpu.pc = cpu.pc.wrapping_add(2);
    }

    fn mvi(cpu: &mut cpu::CPU, to: Reg) {
        let value = cpu.memory[(cpu.pc as usize) + 1];
        Self::write_to_reg(cpu, to, value);
        cpu.pc = cpu.pc.wrapping_add(1);
    }

    fn mov(cpu: &mut cpu::CPU, to: Reg, from: Reg) {
        match from {
            Reg::B => Self::write_to_reg(cpu, to, cpu.b),
            Reg::C => Self::write_to_reg(cpu, to, cpu.c),
            Reg::D => Self::write_to_reg(cpu, to, cpu.d),
            Reg::E => Self::write_to_reg(cpu, to, cpu.e),
            Reg::H => Self::write_to_reg(cpu, to, cpu.h),
            Reg::L => Self::write_to_reg(cpu, to, cpu.l),
            Reg::HL => {
                let addr = (((cpu.h as u16) << 8) | (cpu.l as u16)) as usize;
                Self::write_to_reg(cpu, to, cpu.memory[addr]);
            },
            Reg::A => Self::write_to_reg(cpu, to, cpu.a),
            _ => panic!("MOV CALLED FROM WRONG REG!")
        }
        
    }

    //TODO: Implement flags
    fn add(cpu: &mut cpu::CPU, from: Reg, carry: bool) {
        let mut x = 0;
        if carry {
            x = 1;
        } 
        match from {
            Reg::B => cpu.a = cpu.a.wrapping_add(cpu.b + x),
            Reg::C => cpu.a = cpu.a.wrapping_add(cpu.c + x),
            Reg::D => cpu.a = cpu.a.wrapping_add(cpu.d + x),
            Reg::E => cpu.a = cpu.a.wrapping_add(cpu.e + x),
            Reg::H => cpu.a = cpu.a.wrapping_add(cpu.h + x),
            Reg::L => cpu.a = cpu.a.wrapping_add(cpu.l + x),

            Reg::HL => {
                let addr = (((cpu.h as u16) << 8) | (cpu.l as u16)) as usize;
                cpu.a = cpu.a.wrapping_add(cpu.memory[addr] + x);
            },

            Reg::A => cpu.a = cpu.a.wrapping_add(cpu.a + x),
            _ => panic!("ADD CALLED FROM WRONG REG!")
            
        }
    }

    fn jmp(cpu: &mut cpu::CPU) {
        cpu.pc = ((cpu.memory[(cpu.pc+2) as usize]) as u16) << 8 | (cpu.memory[(cpu.pc+1) as usize]) as u16;
        println!("Jumping to: {:x}",cpu.pc);
        //Decrementing PC as it is incremented at the end of emulate function;
        cpu.pc -= 1;
    }

    pub fn emulate(&self, cpu: &mut cpu::CPU) {
        let opcode: u8 = cpu.memory[cpu.pc as usize];
        println!("op: 0x{:x}, pc: {:x}, sp: {:x}",opcode,cpu.pc,cpu.sp);
        match opcode {
            0x00 => println!(""),
            0x01 => Self::lxi(cpu, Reg::B),
            0x06 => Self::mvi(cpu, Reg::B),
            0x0e => Self::mvi(cpu, Reg::C),
            0x11 => Self::lxi(cpu, Reg::D),
            0x16 => Self::mvi(cpu, Reg::D),
            0x1e => Self::mvi(cpu, Reg::E),
            0x21 => Self::lxi(cpu, Reg::H),
            0x26 => Self::mvi(cpu, Reg::H),
            0x2e => Self::mvi(cpu, Reg::L),
            0x31 => Self::lxi(cpu, Reg::SP),
            0x36 => Self::mvi(cpu, Reg::HL),
            0x3e => Self::mvi(cpu, Reg::A),

            //TODO: Check if extract_argument is consistent for all MOVs (or all opcodes)
            0x40 ..= 0x47 => Self::mov(cpu, Reg::B, Self::extract_argument(opcode)),
            0x48 ..= 0x4f => Self::mov(cpu, Reg::C, Self::extract_argument(opcode)),
            0x50 ..= 0x57 => Self::mov(cpu, Reg::D, Self::extract_argument(opcode)),
            0x58 ..= 0x5f => Self::mov(cpu, Reg::E, Self::extract_argument(opcode)),
            0x60 ..= 0x67 => Self::mov(cpu, Reg::H, Self::extract_argument(opcode)),
            0x68 ..= 0x6f => Self::mov(cpu, Reg::L, Self::extract_argument(opcode)),
            0x70 ..= 0x75 => Self::mov(cpu, Reg::HL, Self::extract_argument(opcode)),
            0x77 => Self::mov(cpu, Reg::HL, Self::extract_argument(opcode)),
            0x78 ..= 0x7f => Self::mov(cpu, Reg::A, Self::extract_argument(opcode)),
            0x80 ..= 0x87 => Self::add(cpu, Self::extract_argument(opcode), false),
            0x88 ..= 0x8f => Self::add(cpu, Self::extract_argument(opcode), cpu.flags.cy),


            0xc3 => Self::jmp(cpu),
            _ => {
                panic!("Unimplemented Opcode: 0x{:x}", opcode);
            }
        }
        cpu.pc = cpu.pc.wrapping_add(1);
    }
}