use crate::{crypto::field::Fq, vm::constants::NUM_DATA_COLUMNS};

/// State of the VM at a single clock cycle.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VMState {
    pub clock_cycle: Fq,
    pub data_columns: [Fq; NUM_DATA_COLUMNS],
    pub control_init: Fq,
    pub control_step: Fq,
    pub control_final: Fq,
}

/// Full execution trace of the VM.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VMTrace {
    pub states: Vec<VMState>,
}
