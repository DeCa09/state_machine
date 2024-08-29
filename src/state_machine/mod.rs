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
    use crate::tests::common::{
        SampleState, SampleStateContext, SampleStateData, SampleStateMachine,
    };

    #[test]
    fn should_return_sample_state_as_current_state() {
        let sample_state_machine = SampleStateMachine::default();

        let expected_result = &SampleState::default();

        let result = sample_state_machine.get_current_state();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_name_of_sample_state_when_state_machine_in_sample_state() {
        let sample_state_machine = SampleStateMachine::default();

        let expected_result = String::from("Sample State");

        let result = sample_state_machine
            .get_current_state()
            .get_state_name()
            .to_string();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_default_state_data_struct_as_input_data_when_output_data_has_not_been_computed_in_state(
    ) {
        let sample_state_machine = SampleStateMachine::default();

        let expected_result = &SampleStateData::default();

        let result = sample_state_machine.get_current_state().get_input_data();

        assert_eq!(result, expected_result);
    }

    #[test]
    #[should_panic(expected = "output should not be empty")]
    fn should_panic_when_trying_to_access_output_data_before_it_has_been_computed_in_state() {
        let sample_state_machine = SampleStateMachine::default();

        let _result = sample_state_machine
            .get_current_state()
            .get_output_data()
            .expect("The output should not be empty.");
    }
}
