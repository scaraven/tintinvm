use crate::{
    constraints::check::{
        is_fibonacci, is_initial_state_column_one, is_initial_state_column_two,
        is_terminated_state, is_transition_data_one, is_transition_data_two,
    },
    crypto::field::Fq,
    vm::{constants::NUM_ROWS, state::VMTrace},
};
use array_init::array_init;

pub struct CheckVMTrace {
    pub is_fibonacci: [Fq; NUM_ROWS],
    pub is_transition_data_one: [Fq; NUM_ROWS],
    pub is_transition_data_two: [Fq; NUM_ROWS],
    pub is_terminated_state: [Fq; NUM_ROWS],
    pub is_initial_state_column_one: [Fq; NUM_ROWS],
    pub is_initial_state_column_two: [Fq; NUM_ROWS],
}

impl CheckVMTrace {
    pub fn new(execution_trace: &VMTrace) -> Self {
        Self {
            is_fibonacci: array_init(|i| is_fibonacci(i, execution_trace)),
            is_transition_data_one: array_init(|i| is_transition_data_one(i, execution_trace)),
            is_transition_data_two: array_init(|i| is_transition_data_two(i, execution_trace)),
            is_terminated_state: array_init(|i| {
                is_terminated_state(i, Fq::from(0u64), execution_trace)
            }),
            is_initial_state_column_one: array_init(|i| {
                is_initial_state_column_one(i, Fq::from(1u64), execution_trace)
            }),
            is_initial_state_column_two: array_init(|i| {
                is_initial_state_column_two(i, Fq::from(1u64), execution_trace)
            }),
        }
    }
}
