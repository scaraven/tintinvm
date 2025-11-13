// Define instruction set for the zkVM

#[derive(Debug, Clone)]
pub enum Instr {
    Const(u64),
    Add,
    Halt,
}

pub const MAX_PROGRAM_SIZE: usize = 4096;

pub struct Program {
    pub instructions: Vec<Instr>,
    pub immediate: [u64; MAX_PROGRAM_SIZE],
}
