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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::{constants::NUM_ROWS, state::VMState};

    fn build_canonical_trace() -> VMTrace {
        let mut states = Vec::with_capacity(NUM_ROWS);

        // Row 0: initial state with inputs 1,1 and Fibonacci-consistent c2
        let c0 = Fq::from(1u64);
        let c1 = Fq::from(1u64);
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
    fn constraints_hold_on_canonical_trace() {
        let trace = build_canonical_trace();

        // Row 0: only initial-state constraints active
        assert_eq!(is_initial_state_column_one(0, Fq::from(1u64), &trace), ZERO);
        assert_eq!(is_initial_state_column_two(0, Fq::from(1u64), &trace), ZERO);
        assert_eq!(is_transition_data_one(0, &trace), ZERO);
        assert_eq!(is_transition_data_two(0, &trace), ZERO);

        // Middle rows where control_step = 1
        for row in 1..(NUM_ROWS - 1) {
            assert_eq!(is_fibonacci(row, &trace), ZERO);
            assert_eq!(is_transition_data_one(row, &trace), ZERO);
            assert_eq!(is_transition_data_two(row, &trace), ZERO);
        }

        // Final row: only final constraint active
        let last = NUM_ROWS - 1;
        assert_eq!(is_fibonacci(last, &trace), ZERO);
        assert_eq!(
            is_terminated_state(last, trace.states[last].data_columns[2], &trace),
            ZERO
        );
    }

    #[test]
    fn violations_with_controls_on_are_detected() {
        let mut trace = build_canonical_trace();

        // Break Fibonacci at some middle row
        let row = 3;
        trace.states[row].data_columns[2] += Fq::from(1u64);
        assert_ne!(is_fibonacci(row, &trace), ZERO);

        // Restore Fibonacci, then break transition for data column 0
        trace = build_canonical_trace();
        let row = 2;
        trace.states[row].data_columns[0] += Fq::from(1u64);
        trace.states[row].control_step = Fq::from(1u64);
        assert_ne!(is_transition_data_one(row, &trace), ZERO);

        // Break initial state while control_init = 1
        trace = build_canonical_trace();
        trace.states[0].data_columns[0] += Fq::from(1u64);
        assert_ne!(is_initial_state_column_one(0, Fq::from(1u64), &trace), ZERO);

        // Break final state while control_final = 1
        trace = build_canonical_trace();
        let last = NUM_ROWS - 1;
        trace.states[last].data_columns[2] += Fq::from(1u64);
        assert_ne!(
            is_terminated_state(
                last,
                trace.states[last].data_columns[2] - Fq::from(1u64),
                &trace
            ),
            ZERO
        );
    }

    #[test]
    fn violations_with_controls_off_are_ignored() {
        let mut trace = build_canonical_trace();

        // Middle row: break transitions but turn control_step off
        let row = 2;
        trace.states[row].data_columns[0] += Fq::from(1u64);
        trace.states[row].data_columns[1] += Fq::from(1u64);
        trace.states[row].control_step = ZERO;
        assert_eq!(is_transition_data_one(row, &trace), ZERO);
        assert_eq!(is_transition_data_two(row, &trace), ZERO);

        // Initial row: wrong values but control_init off
        trace.states[0].data_columns[0] += Fq::from(5u64);
        trace.states[0].data_columns[1] += Fq::from(7u64);
        trace.states[0].control_init = ZERO;
        assert_eq!(is_initial_state_column_one(0, Fq::from(1u64), &trace), ZERO);
        assert_eq!(is_initial_state_column_two(0, Fq::from(1u64), &trace), ZERO);

        // Final row: wrong value but control_final off
        let last = NUM_ROWS - 1;
        trace.states[last].data_columns[2] += Fq::from(3u64);
        trace.states[last].control_final = ZERO;
        assert_eq!(is_terminated_state(last, Fq::from(0u64), &trace), ZERO);
    }
}
