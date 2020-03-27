use flags::Flags;
mod flags;


const MEM_SIZE: usize = 0x10000;

pub struct CPU {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    pub memory: [u8;MEM_SIZE],
    pub sp: u16,
    pub pc: u16,

    pub flags: Flags,
    pub interrupts_enabled: bool,
}

impl CPU {
    //TODO: Write implementation of CPU
    pub fn new() -> CPU {
        CPU {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
        
            memory: [0;MEM_SIZE],
            sp: 0,
            pc: 0,
        
            flags: Flags::new(),
            interrupts_enabled: false,
        }
    }
}

