use emulator::cpu::CPU;
use emulator::Emulator;
mod emulator;


fn main() {
    let mut cpu = CPU::new();
    let emulator = Emulator::new();
    emulator.loadRom(&mut cpu, 0x00);
}
