use crate::{
    crypto::field::{Fq, ZERO},
    vm::state::VMTrace,
};

/// Returns ZERO if the Fibonacci constraint is satisfied at the given row
pub fn is_fibonacci(row: usize, trace: &VMTrace) -> Fq {
    trace.states[row].data_columns[2]
        - (trace.states[row].data_columns[0] + trace.states[row].data_columns[1])
}

/// Returns ZERO iff current data column 0 transitions from previous row column 1
/// or control step is ZERO or it's the first row
pub fn is_transition_data_one(row: usize, trace: &VMTrace) -> Fq {
    if row == 0 {
        return ZERO;
    }

    trace.states[row].control_step
        * (trace.states[row].data_columns[0] - trace.states[row - 1].data_columns[1])
}

pub fn is_transition_data_two(row: usize, trace: &VMTrace) -> Fq {
    if row == 0 {
        return ZERO;
    }

    trace.states[row].control_step
        * (trace.states[row].data_columns[1] - trace.states[row - 1].data_columns[2])
}

pub fn is_terminated_state(row: usize, output: Fq, trace: &VMTrace) -> Fq {
    trace.states[row].control_final * (trace.states[row].data_columns[2] - output)
}

pub fn is_initial_state_column_one(row: usize, input: Fq, trace: &VMTrace) -> Fq {
    trace.states[row].control_init * (trace.states[row].data_columns[0] - input)
}

pub fn is_initial_state_column_two(row: usize, input: Fq, trace: &VMTrace) -> Fq {
    trace.states[row].control_init * (trace.states[row].data_columns[1] - input)
}
