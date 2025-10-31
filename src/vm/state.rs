use crate::vm::instr::Instr;

const MAX_STACK_SIZE: usize = 1024;

pub struct VMState {
    pc: usize,
    stack: [u64; MAX_STACK_SIZE],
    sp: usize,
    instr: Instr,
}

impl VMState {
    pub fn new(pc: usize, instr: Instr, stack: [u64; MAX_STACK_SIZE], sp: usize) -> Self {
        Self {
            pc,
            stack,
            sp,
            instr,
        }
    }
}
