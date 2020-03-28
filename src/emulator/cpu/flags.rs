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
}