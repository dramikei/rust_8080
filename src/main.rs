use cpu::CPU;
mod cpu;


fn main() {
    let mut cpu = CPU::new();
    cpu.a = 3;
    println!("{}",cpu.a);
}
