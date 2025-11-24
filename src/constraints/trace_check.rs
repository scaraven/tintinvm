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
    pub fn new(execution_trace: &VMTrace, output: Fq, input_one: Fq, input_two: Fq) -> Self {
        Self {
            is_fibonacci: array_init(|i| is_fibonacci(i, execution_trace)),
            is_transition_data_one: array_init(|i| is_transition_data_one(i, execution_trace)),
            is_transition_data_two: array_init(|i| is_transition_data_two(i, execution_trace)),
            is_terminated_state: array_init(|i| is_terminated_state(i, output, execution_trace)),
            is_initial_state_column_one: array_init(|i| {
                is_initial_state_column_one(i, input_one, execution_trace)
            }),
            is_initial_state_column_two: array_init(|i| {
                is_initial_state_column_two(i, input_two, execution_trace)
            }),
        }
    }
}

mod tests {
    use super::*;
    use crate::vm::state::VMState;
    use crate::{crypto::field::ZERO, vm::constants::NUM_DATA_COLUMNS};

    fn build_canonical_trace(input_one: Fq, input_two: Fq, _output: Fq) -> VMTrace {
        let mut states = Vec::with_capacity(NUM_ROWS);

        // Row 0: initial state with given inputs and Fibonacci-consistent c2
        let c0 = input_one;
        let c1 = input_two;
        let c2 = c0 + c1;
        states.push(VMState {
            clock_cycle: Fq::from(0u64),
            data_columns: [c0, c1, c2],
            control_init: Fq::from(1u64),
            control_step: ZERO,
            control_final: ZERO,
        });

        // Middle rows: propagate Fibonacci-like sequence with control_step = 1
        for i in 1..NUM_ROWS {
            let prev = &states[i - 1].data_columns;
            let c0 = prev[1];
            let c1 = prev[2];
            let c2 = c0 + c1;

            let (control_step, control_final) = if i + 1 == NUM_ROWS {
                (ZERO, Fq::from(1u64))
            } else {
                (Fq::from(1u64), ZERO)
            };

            states.push(VMState {
                clock_cycle: Fq::from(i as u64),
                data_columns: [c0, c1, c2],
                control_init: ZERO,
                control_step,
                control_final,
            });
        }

        VMTrace { states }
    }

    #[test]
    fn check_vmtrace_new_succeeds_on_canonical_trace() {
        let input_one = Fq::from(1u64);
        let input_two = Fq::from(1u64);
        let output = Fq::from(55u64);

        let trace = build_canonical_trace(input_one, input_two, output);
        let checked = CheckVMTrace::new(&trace, output, input_one, input_two);

        // All rows satisfy Fibonacci
        for row in 0..NUM_ROWS {
            assert_eq!(checked.is_fibonacci[row], ZERO);
        }

        // Initial row: initial state constraints active
        assert_eq!(checked.is_initial_state_column_one[0], ZERO);
        assert_eq!(checked.is_initial_state_column_two[0], ZERO);

        // Middle rows: transitions active
        for row in 1..(NUM_ROWS - 1) {
            assert_eq!(checked.is_transition_data_one[row], ZERO);
            assert_eq!(checked.is_transition_data_two[row], ZERO);
        }

        // Final row: termination constraint active
        let last = NUM_ROWS - 1;
        assert_eq!(checked.is_terminated_state[last], ZERO);

        // Sanity: arrays lengths align with NUM_ROWS and data columns count
        assert_eq!(trace.states.len(), NUM_ROWS);
        assert_eq!(trace.states[0].data_columns.len(), NUM_DATA_COLUMNS);
    }

    #[test]
    fn single_constraint_failure_isolated_in_check_vmtrace() {
        let input_one = Fq::from(1u64);
        let input_two = Fq::from(1u64);
        let output = Fq::from(55u64);

        let mut trace = build_canonical_trace(input_one, input_two, output);

        // Introduce a single Fibonacci violation at row 3
        let row = 3;
        trace.states[row].data_columns[2] += Fq::from(1u64);

        let checked = CheckVMTrace::new(&trace, output, input_one, input_two);

        for i in 0..NUM_ROWS {
            if i == row {
                assert_ne!(checked.is_fibonacci[i], ZERO);
            } else {
                assert_eq!(checked.is_fibonacci[i], ZERO);
            }

            // Other constraints should still hold everywhere
            if i > 0 && i < NUM_ROWS - 1 && i != row + 1 {
                assert_eq!(checked.is_transition_data_one[i], ZERO);
                assert_eq!(checked.is_transition_data_two[i], ZERO);
            }
        }
    }
}
