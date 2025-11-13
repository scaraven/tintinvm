use crate::{crypto::field::Fq, vm::constants::NUM_DATA_COLUMNS};

pub struct VMState {
    pub clock_cycle: Fq,
    pub data_columns: [Fq; NUM_DATA_COLUMNS],
    pub control_init: Fq,
    pub control_step: Fq,
    pub control_final: Fq,
}

pub struct VMTrace {
    pub states: Vec<VMState>,
}
