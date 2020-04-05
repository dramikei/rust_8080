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

    //LSB of Space Invader's external shift hardware
    pub shift0: u8,
    //MSB
    pub shift1: u8,
    // offset for external shift hardware
    pub shift_offset: u8,
    pub in_port1: u8,

    //output ports for sounds
    pub out_port3: u8,
    pub out_port5: u8,
    pub last_out_port3: u8,
    pub last_out_port5: u8,
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
            shift0: 0,
            shift1: 0,
            shift_offset: 0,
            in_port1: 0,
            out_port3: 0,
            out_port5: 0,
            last_out_port3: 0,
            last_out_port5: 0,
        }
    }
}

