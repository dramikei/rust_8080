pub struct Flags {
    z: bool,
    s: bool,
    p: bool,
    cy: bool,
    ac: bool,

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