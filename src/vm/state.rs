use crate::crypto::field::Fq;

const MAX_STACK_SIZE: usize = 1024;

pub struct VMState {
    clock_cycle: Fq,
}
