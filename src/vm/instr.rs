// Define instruction set for the zkVM

#[derive(Debug, Clone)]
pub enum Instr {
    Const(u64),
    Add,
    Halt,
}
