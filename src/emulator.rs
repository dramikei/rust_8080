use cpu::CPU;
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


    pub fn loadRom(&self, cpu: &mut cpu::CPU, start: usize) {
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
            Reg::A => cpu.b = value,
            //TODO: Add implementation for Write_To_(HL)
            _ => panic!("write to HL called from wrong function!")
        }
    }

    fn write_from_HL(cpu: &mut cpu::CPU, to: Reg) {
        //TODO: implement write from HL
        match to {
            _ => {
                panic!("Unimplemented From_HL_To_Reg");
            }
        }
    }

    fn Mov(cpu: &mut cpu::CPU, to: Reg, mut from: Reg) {
        match from {
            Reg::B => Self::write_to_reg(cpu, to, cpu.b),
            Reg::C => Self::write_to_reg(cpu, to, cpu.c),
            Reg::D => Self::write_to_reg(cpu, to, cpu.d),
            Reg::E => Self::write_to_reg(cpu, to, cpu.e),
            Reg::H => Self::write_to_reg(cpu, to, cpu.h),
            Reg::L => Self::write_to_reg(cpu, to, cpu.l),
            Reg::HL => Self::write_from_HL(cpu, to),
            Reg::A => Self::write_to_reg(cpu, to, cpu.a)
        }
        
    }

    fn Jmp(cpu: &mut cpu::CPU) {
        cpu.pc = ((cpu.memory[(cpu.pc+2) as usize]) as u16) << 8 | (cpu.memory[(cpu.pc+1) as usize]) as u16;
        println!("Jumping to: {:x}",cpu.pc);
        //Decrementing PC as it is incremented at the end of emulate function;
        cpu.pc -= 1;
    }

    pub fn emulate(&self, cpu: &mut cpu::CPU) {
        let opcode: u8 = cpu.memory[cpu.pc as usize];
        println!("opcode: 0x{:x}, pc: {:x}",opcode,cpu.pc);
        match opcode {
            0x00 => println!(""),
            
            //TODO: Check if extract_argument is consistent for all MOVs (or all opcodes)
            0x40 ... 0x47 => Self::Mov(cpu, Reg::B, Self::extract_argument(opcode)),
            0x48 ... 0x4f => Self::Mov(cpu, Reg::C, Self::extract_argument(opcode)),
            0x50 ... 0x57 => Self::Mov(cpu, Reg::D, Self::extract_argument(opcode)),
            0x58 ... 0x5f => Self::Mov(cpu, Reg::E, Self::extract_argument(opcode)),
            0x60 ... 0x67 => Self::Mov(cpu, Reg::H, Self::extract_argument(opcode)),
            0x68 ... 0x6f => Self::Mov(cpu, Reg::L, Self::extract_argument(opcode)),
            0x70 ... 0x75 => Self::Mov(cpu, Reg::HL, Self::extract_argument(opcode)),
            0x77 => Self::Mov(cpu, Reg::HL, Self::extract_argument(opcode)),
            0x78 ... 0x7f => Self::Mov(cpu, Reg::A, Self::extract_argument(opcode)),

            0xc3 => Self::Jmp(cpu),
            _ => {
                panic!("Unimplemented Opcode: 0x{:x}", opcode);
            }
        }
        cpu.pc += 1;
    }




}