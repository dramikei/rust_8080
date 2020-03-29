pub struct Flags {
    pub z: bool,
    pub s: bool,
    pub p: bool,
    pub cy: bool,
    pub ac: bool,

}

impl Flags {
    pub fn new() -> Flags {
        Flags {
            z: false,
            s: false,
            p: false,
            cy: false,
            ac: false,
        }
    }


    //Set flags from a byte
    pub fn set_psw(&mut self, psw: u8) {
        self.cy = (psw & 1) != 0;
        self.p = (psw & 1 << 2) != 0;
        self.ac = (psw & 1 << 4) != 0;
        self.z = (psw & 1 << 6) != 0;
        self.s = (psw & 1 << 7) != 0;
    }

    pub fn set_all(&mut self, value: u16, aux_value: u8) {
        self.set_sign(value as u8);
        self.set_zero(value as u8);
        self.set_aux_carry(aux_value);
        self.set_parity(value as u8);
        self.set_carry(value);
    }

    pub fn set_all_but_carry(&mut self, value: u8) {
        self.set_sign(value);
        self.set_zero(value);
        self.set_aux_carry(value);
        self.set_parity(value);
    }

    pub fn set_all_but_aux_carry(&mut self, value: u16) {
        self.set_sign(value as u8);
        self.set_zero(value as u8);
        self.set_parity(value as u8);
        self.set_carry(value);
    }

    fn set_zero(&mut self, value: u8) {
        self.z = value == 0;
    }

    fn set_sign(&mut self, value: u8) {
        self.s = value & (1 << 7) != 0;
    }

    fn set_parity(&mut self, value: u8) {
        self.p = value.count_ones() % 2 == 0;
    }

    pub fn set_carry(&mut self, value: u16) {
        self.cy = value > 0xff;
    }

    pub fn set_carry_u32(&mut self, value: u32) {
        self.cy = value > 0xffff;
    }

    fn set_aux_carry(&mut self, value: u8) {
        self.ac = value > 0xf;
    }
}