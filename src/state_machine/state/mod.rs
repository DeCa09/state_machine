use super::context::ContextData;
use data::StateData;
use std::fmt::Debug;

pub mod data;

pub trait State: Debug {
    fn get_state_name(self) -> String;

    fn get_input_data(self) -> impl StateData;

    fn get_output_data(self) -> impl StateData;
    fn has_output_data_been_computed(self) -> bool;

    fn get_context_data(self) -> impl ContextData;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::common::StartingState;

    const STARTING_STATE: StartingState = StartingState;

    #[test]
    fn should_return_name_of_starting_state_when_in_starting_state() {
        let starting_state = STARTING_STATE;

        let expected_result = String::from("Starting State");

        let result = starting_state.get_state_name();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_simple_string_as_input_data_when_in_starting_state() {
        let starting_state = STARTING_STATE;

        let expected_result = String::from("some input data");

        let result = starting_state.get_input_data();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_simple_string_as_output_data_when_in_starting_state() {
        let starting_state = STARTING_STATE;

        let expected_result = String::from("some output data");

        let result = starting_state.get_output_data();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_true_when_starting_state_has_computed_the_output() {
        let starting_state = STARTING_STATE;

        let expected_result = true;

        let result = starting_state.has_output_data_been_computed();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_false_when_starting_state_has_computed_the_output() {
        let starting_state = STARTING_STATE;

        let expected_result = false;

        let result = starting_state.has_output_data_been_computed();

        assert_eq!(result, expected_result);
    }

    fn should_return_simple_string_as_context_data_when_in_starting_state() {
        let starting_state = STARTING_STATE;

        let expected_result = String::from("some context data");

        let result = starting_state.get_output_data();

        assert_eq!(result, expected_result);
    }
}
