use self::cpu::OLC6502;

mod bus;
mod cpu;
mod memory;

fn main() {
    let mut bus = bus::Bus::new();
    let mut ram = memory::RAM::new();
    bus.add_device(&mut ram);
    let mut cpu = OLC6502::new(&mut bus);
    cpu.write(0xFFFE, 15);
    println!("{}", cpu);
    println!("{:?}", cpu.read(0xFFFE));
}
