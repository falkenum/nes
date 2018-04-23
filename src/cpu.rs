
#[allow(dead_code)] // TODO delete
#[derive(Debug)]
pub struct CPU {
    a : u8,
    x : u8,
    y : u8,
    sp : u8,
    pc : u16,
    flags : u8,
}

trait AddrMode {
    fn exec(InstrArg, &CPU);
}
// instruction args are only up to 2 bytes
type InstrArg = u16;
fn exec_op (cpu : &CPU, op : u8, arg : InstrArg) {

    fn foo() {};

    // gen_instr!(&cpu, u)
    /// test
    const instr : [fn(); 2] = [|| (), || ()]; /* modify regs or mem */
    instr[op as usize]();
}

mod instructions {

}

pub struct RAM {}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            a : 0,
            x : 0,
            y : 0,
            sp : 0,
            pc : 0,
            flags : 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test0() {
        let c = CPU::new();
        println!("{:#?}", c);
    }
}
