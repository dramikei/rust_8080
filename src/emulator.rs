// use cpu::CPU;
use register::Reg;
use std;
pub mod cpu;
mod register;

pub struct Emulator {
    pub cycles8080: [u128;256],
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

    pub fn generate_interrupt(&self, cpu: &mut cpu::CPU, interrupt_num: u32) {
        // println!("Pushing to Stack(Interrupt): {:04x}",cpu.pc);
        Self::push_to_stack_addr(cpu, cpu.pc);
        cpu.pc = 8*(interrupt_num as u16);
        // println!("PC is at: {:04x}",cpu.pc);
        cpu.interrupts_enabled = false;
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
            _ => panic!("Could not resolve operating register for opcode {:02x}",opcode),
        }
    }

    fn write_to_reg(cpu: &mut cpu::CPU, to: Reg, value: u8) {
        match to {
            Reg::B => cpu.b = value,
            Reg::C => cpu.c = value,
            Reg::D => cpu.d = value,
            Reg::E => cpu.e = value,
            Reg::H => cpu.h = value,
            Reg::L => cpu.l = value,
            Reg::HL => {
                let addr = (((cpu.h as u16) << 8) | (cpu.l as u16)) as usize;
                cpu.memory[addr] = value;
            },
            Reg::A => cpu.a = value,
            _ => panic!("write-to-reg CALLED ON WRONG REG!")
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

    fn ldax(cpu: &mut cpu::CPU, from: Reg) {
        match from {
            Reg::B => {
                let addr = ((cpu.b as u16) << 8) | cpu.c as u16;
                cpu.a = cpu.memory[addr as usize];
            },
            Reg::D => {
                let addr = ((cpu.d as u16) << 8) | cpu.e as u16;
                cpu.a = cpu.memory[addr as usize];
            },
            _ => {
                panic!("LDAX CALLED ON WRONG REG!");
            }
        }
    }

    fn stax(cpu: &mut cpu::CPU, from: Reg) {
        match from {
            Reg::B => {
                let addr = ((cpu.b as u16) << 8) | cpu.c as u16;
                cpu.memory[addr as usize] = cpu.a;
            },
            Reg::D => {
                let addr = ((cpu.d as u16) << 8) | cpu.e as u16;
                cpu.memory[addr as usize] = cpu.a;
            },
            _ => {
                panic!("STAX CALLED ON WRONG REG!");
            }
        }
    }



    //ADD function is also used by ADC instruction (ADD with Carry). (Thats why 3rd parameter of 'carry' exists)
    //Flags Affected: All
    fn add(cpu: &mut cpu::CPU, from: Reg, carry: bool) {
        let result: u16;
        let operand: u8;
        match from {
            Reg::B => operand = cpu.b,
            Reg::C => operand = cpu.c,
            Reg::D => operand = cpu.d,
            Reg::E => operand = cpu.e,
            Reg::H => operand = cpu.h,
            Reg::L => operand = cpu.l,

            Reg::HL => {
                let addr = (((cpu.h as u16) << 8) | (cpu.l as u16)) as usize;
                operand = cpu.memory[addr];
            },

            Reg::A => operand = cpu.a,
            _ => panic!("ADD CALLED FROM WRONG REG!")
        }
        result = (cpu.a as u16).wrapping_add(operand as u16).wrapping_add(carry as u16);
        cpu.flags.set_all(result, (cpu.a & 0xf).wrapping_add(operand.wrapping_add(carry as u8) & 0xf));
        cpu.a = result as u8;
    }


    //SUB function is also used by SBB instruction. (Thats why 3rd parameter of 'carry' exists)
    //Flags Affected: ALL
    fn sub(cpu: &mut cpu::CPU, from: Reg, carry: bool) {
        let result: u16;
        let operand: u8;
        match from {
            Reg::B => operand = cpu.b,
            Reg::C => operand = cpu.c,
            Reg::D => operand = cpu.d,
            Reg::E => operand = cpu.e,
            Reg::H => operand = cpu.h,
            Reg::L => operand = cpu.l,

            Reg::HL => {
                let addr = (((cpu.h as u16) << 8) | (cpu.l as u16)) as usize;
                operand = cpu.memory[addr];
            },

            Reg::A => operand = cpu.a,
            _ => panic!("SUB CALLED FROM WRONG REG!")
        }
        result = (cpu.a as u16).wrapping_sub(operand as u16).wrapping_sub(carry as u16);
        cpu.flags.set_all(result, (cpu.a & 0xf).wrapping_sub(operand.wrapping_sub(carry as u8) & 0xf));
        cpu.a = result as u8;
    }


    //Carry is reset to zero. Flags affected: Carry, Zero, Sign, Parity.
    fn ana(cpu: &mut cpu::CPU, reg: Reg) {
        let result: u16;
        let operand: u8;
        cpu.flags.cy = false;
        match reg {
            Reg::B => operand = cpu.b,
            Reg::C => operand = cpu.c,
            Reg::D => operand = cpu.d,
            Reg::E => operand = cpu.e,
            Reg::H => operand = cpu.h,
            Reg::L => operand = cpu.l,

            Reg::HL => {
                let addr = (((cpu.h as u16) << 8) | (cpu.l as u16)) as usize;
                operand = cpu.memory[addr];
            },

            Reg::A => operand = cpu.a,
            _ => {
                panic!("ANA Called on wrong REG!");
            }
        }
        result = cpu.a as u16 & operand as u16;
        cpu.flags.set_all_but_aux_carry(result);
        cpu.a = result as u8;

    }

    //Add support for Aux Carry
    fn inr(cpu: &mut cpu::CPU, reg:Reg) {
        let result: u16;
        match reg {
            Reg::B => {
                result = cpu.b.wrapping_add(1) as u16;
                cpu.b = result as u8;
            },
            Reg::C => {
                result = cpu.c.wrapping_add(1) as u16;
                cpu.c = result as u8;
            },
            Reg::D => {
                result = cpu.d.wrapping_add(1) as u16;
                cpu.d = result as u8;
            },
            Reg::E => {
                result = cpu.e.wrapping_add(1) as u16;
                cpu.e = result as u8;
            },
            Reg::H => {
                result = cpu.h.wrapping_add(1) as u16;
                cpu.h = result as u8;
            },
            Reg::L => {
                result = cpu.l.wrapping_add(1) as u16;
                cpu.l = result as u8;
            },

            Reg::HL => {
                let addr = (((cpu.h as u16) << 8) | (cpu.l as u16)) as usize;
                result = cpu.memory[addr].wrapping_add(1) as u16;
                cpu.memory[addr] = result as u8;
            },

            Reg::A => {
                result = cpu.a.wrapping_add(1) as u16;
                cpu.a = result as u8;
            },
            _ => panic!("INR CALLED ON WRONG REG!")
        }
        cpu.flags.set_all_but_aux_carry(result);
    }

    fn inx(cpu: &mut cpu::CPU, reg: Reg) {
        match reg {
            Reg::B => {
                cpu.c = cpu.c.wrapping_add(1);
                if cpu.c == 0 {
                    cpu.b = cpu.b.wrapping_add(1);
                }
            },
            Reg::D => {
                cpu.e = cpu.e.wrapping_add(1);
                if cpu.e == 0 {
                    cpu.d = cpu.d.wrapping_add(1);
                }
            },
            Reg::H => {
                cpu.l = cpu.l.wrapping_add(1);
                if cpu.l == 0 {
                    cpu.h = cpu.h.wrapping_add(1);
                }
            },
            Reg::SP => cpu.sp = cpu.sp.wrapping_add(1),
            _ => {
                panic!("INX CALLED ON WRONG REG!");
            }
        }
    }

    fn dcx(cpu: &mut cpu::CPU, reg: Reg) {
        match reg {
            Reg::B => {
                cpu.c = cpu.c.wrapping_sub(1);
                if cpu.c == 0 {
                    cpu.b = cpu.b.wrapping_sub(1);
                }
            },
            Reg::D => {
                cpu.e = cpu.e.wrapping_sub(1);
                if cpu.e == 0 {
                    cpu.d = cpu.d.wrapping_sub(1);
                }
            },
            Reg::H => {
                cpu.l = cpu.l.wrapping_sub(1);
                if cpu.l == 0 {
                    cpu.h = cpu.h.wrapping_sub(1);
                }
            },
            Reg::SP => cpu.sp = cpu.sp.wrapping_sub(1),
            _ => {
                panic!("DCX CALLED ON WRONG REG!");
            }
        }
    }

    fn dcr(cpu: &mut cpu::CPU, reg: Reg) {
        let result: u8;
        match reg {
            Reg::B => {
                cpu.b = cpu.b.wrapping_sub(1);
                result = cpu.b;
            },
            Reg::C => {
                cpu.c = cpu.c.wrapping_sub(1);
                result = cpu.c;
            },
            Reg::D => {
                cpu.d = cpu.d.wrapping_sub(1);
                result = cpu.d;
            },
            Reg::E => {
                cpu.e = cpu.e.wrapping_sub(1);
                result = cpu.e;
            },
            Reg::H => {
                cpu.h = cpu.h.wrapping_sub(1);
                result = cpu.h;
            },
            Reg::L => {
                cpu.l = cpu.l.wrapping_sub(1);
                result = cpu.l;
            },

            Reg::HL => {
                let addr = (((cpu.h as u16) << 8) | (cpu.l as u16)) as usize;
                cpu.memory[addr] = cpu.memory[addr].wrapping_sub(1);
                result = cpu.memory[addr];
            },

            Reg::A => {
                cpu.a = cpu.a.wrapping_sub(1);
                result = cpu.a;
            },
            _ => {
                panic!("XRA Called on wrong REG!");
            }
        }
        cpu.flags.set_all_but_carry(result);
    }


    //Carry is reset to zero. Flags affected: Carry, Zero, Sign, Parity.
    fn xra(cpu: &mut cpu::CPU, reg: Reg) {
        let result: u16;
        let operand: u8;
        cpu.flags.cy = false;
        match reg {
            Reg::B => operand = cpu.b,
            Reg::C => operand = cpu.c,
            Reg::D => operand = cpu.d,
            Reg::E => operand = cpu.e,
            Reg::H => operand = cpu.h,
            Reg::L => operand = cpu.l,

            Reg::HL => {
                let addr = (((cpu.h as u16) << 8) | (cpu.l as u16)) as usize;
                operand = cpu.memory[addr];
            },

            Reg::A => operand = cpu.a,
            _ => {
                panic!("XRA Called on wrong REG!");
            }
        }
        result = cpu.a as u16 ^ operand as u16;
        //TODO: Check if Aux Carry is affected or not.
        cpu.flags.set_all_but_aux_carry(result);
        cpu.a = result as u8;

    }

     //Carry is reset to zero. Flags affected: Carry, Zero, Sign, Parity.
     fn ora(cpu: &mut cpu::CPU, reg: Reg) {
        let result: u16;
        let operand: u8;
        cpu.flags.cy = false;
        match reg {
            Reg::B => operand = cpu.b,
            Reg::C => operand = cpu.c,
            Reg::D => operand = cpu.d,
            Reg::E => operand = cpu.e,
            Reg::H => operand = cpu.h,
            Reg::L => operand = cpu.l,

            Reg::HL => {
                let addr = (((cpu.h as u16) << 8) | (cpu.l as u16)) as usize;
                operand = cpu.memory[addr];
            },

            Reg::A => operand = cpu.a,
            _ => {
                panic!("ORA Called on wrong REG!");
            }
        }
        result = cpu.a as u16 | operand as u16;
        //TODO: Check if Aux Carry is affected or not.
        cpu.flags.set_all_but_aux_carry(result);
        cpu.a = result as u8;

    }

    fn dad(cpu: &mut cpu::CPU, reg: Reg) {
        let result: u32;
        match reg {
            Reg::B => result = (((cpu.h as u32) <<8) | (cpu.l as u32)) + (((cpu.b as u32) <<8) | (cpu.c as u32)),
            Reg::D => result = (((cpu.h as u32) <<8) | (cpu.l as u32)) + (((cpu.d as u32) <<8) | (cpu.e as u32)),
            Reg::H => result = (((cpu.h as u32) <<8) | (cpu.l as u32)) + (((cpu.h as u32) <<8) | (cpu.l as u32)),
            Reg::SP => result = (((cpu.h as u32) <<8) | (cpu.l as u32)) + cpu.sp as u32,
            _ => {
                panic!("DAD CANNED ON WRONG REG!");
            }
        }
        cpu.flags.set_carry_u32(result);
        cpu.l = result as u8;
        cpu.h = (result >> 8) as u8;
    }

    fn cmp(cpu: &mut cpu::CPU, reg: Reg) {
        let operand: u8;
        match reg {
            Reg::B => operand = cpu.b,
            Reg::C => operand = cpu.c,
            Reg::D => operand = cpu.d,
            Reg::E => operand = cpu.e,
            Reg::H => operand = cpu.h,
            Reg::L => operand = cpu.l,
            Reg::HL => {
                let addr = (((cpu.h as u16) << 8) | (cpu.l as u16)) as usize;
                operand = cpu.memory[addr];
            },
            Reg::A => operand = cpu.a,
            _ => panic!("CMP CALLED ON WRONG REG!")
        }
        cpu.flags.set_all((cpu.a as u16).wrapping_sub(operand as u16), (cpu.a & 0xf).wrapping_sub(operand & 0xf));
    }

    fn jmp(cpu: &mut cpu::CPU) {
        cpu.pc = ((cpu.memory[(cpu.pc+2) as usize]) as u16) << 8 | (cpu.memory[(cpu.pc+1) as usize]) as u16;
        // println!("Jumping to: {:04x}",cpu.pc);
        //Decrementing PC as it is incremented at the end of emulate function;
        cpu.pc -= 1;
    }

    fn jmpt_to(cpu: &mut cpu::CPU, addr: u16) {
        cpu.pc = addr;
        // println!("Jumping to: {:04x}",addr);
        //Decrementing PC as it is incremented at the end of emulate function;
        cpu.pc -= 1;
    }

    fn call(cpu: &mut cpu::CPU) {
        // println!("Instruction CALL called");
        Self::push_to_stack_addr(cpu, cpu.pc.wrapping_add(3));
        Self::jmp(cpu);
    }

    fn ret(cpu: &mut cpu::CPU) {
        // println!("Instruction RET called");
        let addr = Self::pop_from_stack(cpu);
        Self::jmpt_to(cpu, addr);
    }

    fn push(cpu: &mut cpu::CPU, reg: Reg) {
        match reg {
            Reg::B => {
                let addr = ((cpu.b as u16) << 8) | cpu.c as u16;
                Self::push_to_stack_addr(cpu, addr);
            },
            Reg::D => {
                let addr = ((cpu.d as u16) << 8) | cpu.e as u16;
                Self::push_to_stack_addr(cpu, addr);
            },
            Reg::H => {
                let addr = ((cpu.h as u16) << 8) | cpu.l as u16;
                Self::push_to_stack_addr(cpu, addr);
            },
            _ => {
                panic!("PUSH called at Wrong REG!");
            }
        }

    }

    fn push_psw(cpu: &mut cpu::CPU) {
        // cpu.memory[cpu.sp as us -1] = cpu.a;
        let mut psw:u16 = 0;
        let s = if cpu.flags.s { 1 } else { 0 };
        let z = if cpu.flags.z { 1 } else { 0 };
        let ac = if cpu.flags.ac { 1 } else { 0 };
        let p = if cpu.flags.p { 1 } else { 0 };
        let cy = if cpu.flags.cy { 1 } else { 0 };
        
		psw |= s << 7;
		psw |= z << 6;
		psw |= 0 << 5;
		psw |= ac << 4;
		psw |= 0 << 3;
		psw |= p << 2;
		psw |= 1 << 1;
        psw |= cy;
        psw |= (cpu.a as u16) << 8;
        // println!("Pushing PSW to Stack: {:04x}, cpu.a: {:x}",psw,cpu.a);
        Self::push_to_stack_addr(cpu, psw);
    }

    fn push_to_stack_addr(cpu: &mut cpu::CPU, addr : u16) {
        // println!("Pushing to stack addr: {:04x}",addr);
        cpu.memory[cpu.sp as usize - 1] = (addr >> 8) as u8;
        cpu.memory[cpu.sp as usize - 2] = addr as u8;
        cpu.sp = cpu.sp.wrapping_sub(2);
    }

    fn pop_from_stack(cpu: &mut cpu::CPU) -> u16 {
        let addr: u16;
        addr = ((cpu.memory[cpu.sp as usize + 1] as u16) << 8) | (cpu.memory[cpu.sp as usize] as u16);
        cpu.sp = cpu.sp.wrapping_add(2);
        addr
    }

    fn pop_psw(cpu: &mut cpu::CPU) {
        let data = Self::pop_from_stack(cpu);
        cpu.a = (data >> 8) as u8;
        cpu.flags.set_psw(data as u8);
    }

    fn pop(cpu: &mut cpu::CPU, reg: Reg) {
        let data = Self::pop_from_stack(cpu);
        match reg {
            Reg::B => {
                cpu.c = data as u8;
                cpu.b = (data >> 8) as u8;
            },
            Reg::D => {
                cpu.e = data as u8;
                cpu.d = (data >> 8) as u8;
            },
            Reg::H => {
                cpu.l = data as u8;
                cpu.h = (data >> 8) as u8;
            },
            _ => panic!("POP CALLED ON WRONG REG!")
        }
    }

    fn emu_in(cpu: &mut cpu::CPU) {
        let port = cpu.memory[(cpu.pc as usize) + 1];
        match port {
            0 => cpu.a = 0xf,
            1 => cpu.a = cpu.in_port1,
            2 => cpu.a = cpu.in_port2,
            3 => {
                let v = ((cpu.shift1 as u16) << 8) | (cpu.shift0 as u16);
                cpu.a = (v >> (8-(cpu.shift_offset as u16))) as u8;
            }
            _ => cpu.a = 0
        }
        cpu.pc = cpu.pc.wrapping_add(1);
    }

    fn out(cpu: &mut cpu::CPU) {
        let port = cpu.memory[cpu.pc.wrapping_add(1) as usize];
        let value = cpu.a;
        match port {
            2 => cpu.shift_offset = value & 0x7,
            3 => cpu.out_port3 = value,
            4 => {
                cpu.shift0 = cpu.shift1;
                cpu.shift1 = value;
            },
            5 => cpu.out_port5 = value,
            6 => {},
            _ => panic!("CANNOT WRITE TO PORT: {}",port)
        }
        cpu.pc = cpu.pc.wrapping_add(1);
    }

    fn daa(cpu: &mut cpu::CPU) {
        let mut result = cpu.a as u16;
        let lsb = result & 0xf;
        if cpu.flags.ac || lsb > 9 {
            result += 6;
            if result & 0xf < lsb { cpu.flags.ac = true }
        }
        let lsb = result & 0xf;
        let mut msb = (result >> 4) & 0xf;
        if cpu.flags.cy || msb > 9 { msb += 6 }
        let result = (msb << 4) | lsb;
        cpu.flags.set_all_but_aux_carry(result);
        cpu.a = result as u8;
    }

    pub fn emulate(&self, cpu: &mut cpu::CPU) -> u128 {
        let opcode: u8 = cpu.memory[cpu.pc as usize];
        // println!("op: 0x{:02x}, pc: {:04x}, Z: {}, S: {}, P: {}, CY: {}, AC: {}, sp: {:04x}, interrupt: {}",opcode,cpu.pc,cpu.flags.z,cpu.flags.s,cpu.flags.p,cpu.flags.cy,cpu.flags.ac,cpu.sp,cpu.interrupts_enabled);
        match opcode {
            0x00 => print!(""),
            0x01 => Self::lxi(cpu, Reg::B),
            0x02 => Self::stax(cpu, Reg::B),
            0x03 => Self::inx(cpu, Reg::B),
            0x04 => Self::inr(cpu, Reg::B),
            0x05 => Self::dcr(cpu, Reg::B),
            0x06 => Self::mvi(cpu, Reg::B),
            0x07 => {
                let bit7: u8 = cpu.a & (1 << 7);
                cpu.a <<= 1;
                cpu.a |= bit7 >> 7;
                cpu.flags.cy = bit7 != 0;
            }
            0x09 => Self::dad(cpu, Reg::B),
            0x0a => Self::ldax(cpu, Reg::B),
            0x0b => Self::dcx(cpu, Reg::B),
            0x0c => Self::inr(cpu, Reg::C),
            0x0d => Self::dcr(cpu, Reg::C),
            0x0e => Self::mvi(cpu, Reg::C),
            0x0f => {
                let bit0: u8 = cpu.a & 1;
                cpu.a >>= 1;
                cpu.a |= bit0 << 7;
                cpu.flags.cy = bit0 != 0;
            },
            0x11 => Self::lxi(cpu, Reg::D),
            0x12 => Self::stax(cpu, Reg::D),
            0x13 => Self::inx(cpu, Reg::D),
            0x14 => Self::inr(cpu, Reg::D),
            0x15 => Self::dcr(cpu, Reg::D),
            0x16 => Self::mvi(cpu, Reg::D),
            0x19 => Self::dad(cpu, Reg::D),
            0x1a => Self::ldax(cpu, Reg::D),
            0x1b => Self::dcx(cpu, Reg::D),
            0x1c => Self::inr(cpu, Reg::E),
            0x1d => Self::dcr(cpu, Reg::E),
            0x1e => Self::mvi(cpu, Reg::E),
            0x1f => {
                let bit0: u8 = cpu.a & 1;
                cpu.a >>= 1;
                if cpu.flags.cy { cpu.a |= 1 << 7; }
                cpu.flags.cy = bit0 != 0;
            }
            0x21 => Self::lxi(cpu, Reg::H),
            0x22 => {
                let addr = ((cpu.memory[(cpu.pc as usize) + 2] as u16) << 8) | cpu.memory[(cpu.pc as usize) + 1] as u16;
                cpu.memory[addr as usize] = cpu.l;
                cpu.memory[(addr as usize) + 1] = cpu.h;
                cpu.pc = cpu.pc.wrapping_add(2);
            },
            0x23 => Self::inx(cpu, Reg::H),
            0x24 => Self::inr(cpu, Reg::H),
            0x25 => Self::dcr(cpu, Reg::H),
            0x26 => Self::mvi(cpu, Reg::H),
            0x27 => Self::daa(cpu),
            0x29 => Self::dad(cpu, Reg::H),
            0x2a => {
                let addr = ((cpu.memory[(cpu.pc as usize) + 2] as u16) << 8) | cpu.memory[(cpu.pc as usize) + 1] as u16;
                cpu.l = cpu.memory[addr as usize];
                cpu.h = cpu.memory[addr as usize + 1];
                cpu.pc = cpu.pc.wrapping_add(2);
            },
            0x2b => Self::dcx(cpu, Reg::H),
            0x2c => Self::inr(cpu, Reg::L),
            0x2d => Self::dcr(cpu, Reg::L),
            0x2e => Self::mvi(cpu, Reg::L),
            0x2f => cpu.a = !cpu.a,
            0x31 => Self::lxi(cpu, Reg::SP),
            0x32 => {
                let addr = ((cpu.memory[(cpu.pc as usize) + 2] as u16) << 8) | cpu.memory[(cpu.pc as usize) + 1] as u16;
                cpu.memory[addr as usize] = cpu.a;
                cpu.pc = cpu.pc.wrapping_add(2);
            },
            0x33 => Self::inx(cpu, Reg::SP),
            0x34 => Self::inr(cpu, Reg::HL),
            0x35 => Self::dcr(cpu, Reg::HL),
            0x36 => Self::mvi(cpu, Reg::HL),
            0x37 => cpu.flags.cy = true,
            0x39 => Self::dad(cpu, Reg::SP),
            0x3a => {
                let addr = ((cpu.memory[(cpu.pc as usize) + 2] as u16) << 8) | cpu.memory[(cpu.pc as usize) + 1] as u16;
                cpu.a = cpu.memory[addr as usize];
                cpu.pc = cpu.pc.wrapping_add(2);
            },
            0x3b => Self::dcx(cpu, Reg::SP),
            0x3c => Self::inr(cpu, Reg::A),
            0x3d => Self::dcr(cpu, Reg::A),
            0x3e => Self::mvi(cpu, Reg::A),
            0x3f => cpu.flags.cy = !cpu.flags.cy,

            0x40 ..= 0x47 => Self::mov(cpu, Reg::B, Self::extract_argument(opcode)),
            0x48 ..= 0x4f => Self::mov(cpu, Reg::C, Self::extract_argument(opcode)),
            0x50 ..= 0x57 => Self::mov(cpu, Reg::D, Self::extract_argument(opcode)),
            0x58 ..= 0x5f => Self::mov(cpu, Reg::E, Self::extract_argument(opcode)),
            0x60 ..= 0x67 => Self::mov(cpu, Reg::H, Self::extract_argument(opcode)),
            0x68 ..= 0x6f => Self::mov(cpu, Reg::L, Self::extract_argument(opcode)),
            0x70 ..= 0x75 => Self::mov(cpu, Reg::HL, Self::extract_argument(opcode)),

            0x76 => {
                println!("OPCODE: 0x76 (HALT) Called. Killing Emulator...");
                std::process::exit(0);
            }

            0x77 => Self::mov(cpu, Reg::HL, Self::extract_argument(opcode)),
            0x78 ..= 0x7f => Self::mov(cpu, Reg::A, Self::extract_argument(opcode)),
            0x80 ..= 0x87 => Self::add(cpu, Self::extract_argument(opcode), false),
            0x88 ..= 0x8f => Self::add(cpu, Self::extract_argument(opcode), cpu.flags.cy),
            0x90 ..= 0x97 => Self::sub(cpu, Self::extract_argument(opcode), false),
            0x98 ..= 0x9f => Self::sub(cpu, Self::extract_argument(opcode), cpu.flags.cy),
            0xA0 ..= 0xA7 => Self::ana(cpu, Self::extract_argument(opcode)),
            0xA8 ..= 0xAf => Self::xra(cpu, Self::extract_argument(opcode)),
            0xb0 ..= 0xb7 => Self::ora(cpu, Self::extract_argument(opcode)),
            0xb8 ..= 0xbf => Self::cmp(cpu, Self::extract_argument(opcode)), 
            0xc0 => if !cpu.flags.z { Self::ret(cpu) },
            0xc1 => Self::pop(cpu, Reg::B),
            0xc2 => if !cpu.flags.z { Self::jmp(cpu) } else { cpu.pc = cpu.pc.wrapping_add(2) },
            0xc3 => Self::jmp(cpu),
            0xc4 => if !cpu.flags.z { Self::call(cpu) } else { cpu.pc = cpu.pc.wrapping_add(2) },
            0xc5 => Self::push(cpu, Reg::B),
            0xc6 => {
                let data = cpu.memory[(cpu.pc as usize) + 1];
                let result = cpu.a as u16 + data as u16;
                cpu.flags.set_all(result, (cpu.a & 0xf).wrapping_add(data & 0xf));
                cpu.a = result as u8;
                cpu.pc = cpu.pc.wrapping_add(1);
            },
            0xc8 => if cpu.flags.z { Self::ret(cpu) },
            0xc9 => Self::ret(cpu),
            0xca => if cpu.flags.z { Self::jmp(cpu) } else { cpu.pc = cpu.pc.wrapping_add(2) },
            0xcc => if cpu.flags.z { Self::call(cpu) } else { cpu.pc = cpu.pc.wrapping_add(2) },
            0xcd => Self::call(cpu),

            0xd0 => if !cpu.flags.cy { Self::ret(cpu) },
            0xd1 => Self::pop(cpu, Reg::D),
            0xd2 => if !cpu.flags.cy { Self::jmp(cpu) } else { cpu.pc = cpu.pc.wrapping_add(2) },
            0xd3 => Self::out(cpu),
            0xd4 => if !cpu.flags.cy { Self::call(cpu) } else { cpu.pc = cpu.pc.wrapping_add(2) },
            0xd5 => Self::push(cpu, Reg::D),
            0xd6 => {
                let data = cpu.memory[(cpu.pc as usize) + 1];
                let result = (cpu.a as u16).wrapping_sub( data as u16);
                cpu.flags.set_all(result, (cpu.a & 0xf).wrapping_sub(data & 0xf));
                cpu.a = result as u8;
                cpu.pc = cpu.pc.wrapping_add(1);
            }
            0xd8 => if cpu.flags.cy { Self::ret(cpu) },
            0xda => if cpu.flags.cy { Self::jmp(cpu) } else { cpu.pc = cpu.pc.wrapping_add(2) },
            0xdb => Self::emu_in(cpu),
            0xdc => if cpu.flags.cy { Self::call(cpu) } else { cpu.pc = cpu.pc.wrapping_add(2) },
            0xde => {
                let result: u16;
                let operand: u8 = cpu.memory[(cpu.pc as usize) + 1];
                result = (cpu.a as u16).wrapping_sub(operand as u16).wrapping_sub(cpu.flags.cy as u16);
                cpu.flags.set_all(result, (cpu.a & 0xf).wrapping_sub(operand.wrapping_sub(cpu.flags.cy as u8) & 0xf));
                cpu.a = result as u8;
                cpu.pc = cpu.pc.wrapping_add(1);
            }

            0xe1 => Self::pop(cpu, Reg::H),
            0xe3 => {
                let temp:u16 = ((cpu.h as u16) << 8) | (cpu.l as u16);
                let temp2: u16 = Self::pop_from_stack(cpu);
                cpu.h = (temp2 >> 8) as u8;
                cpu.l = temp2 as u8;
                Self::push_to_stack_addr(cpu, temp);
            },
            0xe5 => Self::push(cpu, Reg::H),
            0xe6 => {
                let data = cpu.memory[(cpu.pc as usize) + 1];
                let result = cpu.a as u16 & data as u16;
                cpu.flags.set_all_but_aux_carry(result);
                cpu.a = result as u8;
                cpu.pc = cpu.pc.wrapping_add(1);
            },
            0xe9 => {
                let addr: u16 = (cpu.h as u16) << 8 | (cpu.l as u16);
                Self::jmpt_to(cpu, addr);
            }
            0xeb => {
                let temp1 = cpu.h;
                let temp2 = cpu.l;
                cpu.h = cpu.d;
                cpu.l = cpu.e;
                cpu.d = temp1;
                cpu.e = temp2;
            },

            0xf0 => if cpu.flags.p { Self::ret(cpu) },
            0xf1 => Self::pop_psw(cpu),
            0xf2 => if cpu.flags.p { Self::jmp(cpu) } else { cpu.pc = cpu.pc.wrapping_add(2) },
            0xf3 => cpu.interrupts_enabled = false,
            0xf5 => Self::push_psw(cpu),
            0xf6 => {
                cpu.a |= cpu.memory[cpu.pc as usize + 1];
                cpu.flags.set_all_but_aux_carry(cpu.a as u16);
                cpu.pc = cpu.pc.wrapping_add(1);
            },
            0xfa => if cpu.flags.s { Self::jmp(cpu) } else { cpu.pc = cpu.pc.wrapping_add(2) },
            0xfb => cpu.interrupts_enabled = true,
            0xfe => {
                let operand = cpu.memory[(cpu.pc as usize) + 1];
                cpu.flags.set_all((cpu.a as u16).wrapping_sub(operand as u16), (cpu.a & 0xf).wrapping_sub(operand & 0xf));
                cpu.pc = cpu.pc.wrapping_add(1);
            }
            _ => {
                //To see changes in screen before panic.
                std::thread::sleep(std::time::Duration::from_secs(10));
                panic!("Unimplemented Opcode: 0x{:02x}", opcode);
            }
        }
        cpu.pc = cpu.pc.wrapping_add(1);
        return self.cycles8080[(opcode)as usize];
    }
}