pub mod state;
pub mod transition;

use crate::state_machine::state::State;

pub trait StateMachine<S: State> {
    fn get_current_state(&self) -> &S;

    fn run(&mut self);

    fn advance_state(&mut self);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::common::{SampleState, SampleStateMachine};

    #[test]
    fn should_return_sample_state_as_current_state() {
        let sample_state_machine = SampleStateMachine::new();

        let expected_result = &SampleState::default();

        let result = sample_state_machine.get_current_state();

        assert_eq!(result, expected_result);
    }
}
