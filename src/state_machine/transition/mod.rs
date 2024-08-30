use crate::state_machine::{state::State, StateMachine};

pub trait Transition<T: State, U: State>: StateMachine<T> + Sized {
    type NextState: State;
    type NextStateMachine: StateMachine<Self::NextState>;

    //type Error;

    /// Transitions the state machine to the next state.
    ///
    /// # Errors
    ///
    /// This function will return an error if the transition to the next state fails.
    /// Possible reasons include:
    /// - Invalid state transition due to undefined
    /// - No conversion between data formats between states possible
    /// - ...
    //fn transition_to_next_state(self) -> Result<Self::NextState, &'static str>;

    fn transition_to_next_state(self) -> Result<Self::NextStateMachine, &'static str>;
    //fn transition_to_next_state(self) -> Result<StateMachine<Self::NextState>, Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state_machine::transition::Transition;
    use crate::tests::common::{ComplexStateMachine, FirstState, SecondState};

    #[test]
    fn should_transition_to_second_state_when_in_first_state() {
        let complex_state_machine = ComplexStateMachine::new();

        let expected_result = String::from("Second State");

        let result =
            Transition::<FirstState, SecondState>::transition_to_next_state(complex_state_machine)
                .expect("Should not fail the transitions to 'SecondState'.")
                .get_current_state()
                .get_state_name()
                .to_string();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_transition_to_first_state_when_in_first_state() {
        let complex_state_machine = ComplexStateMachine::new();

        let expected_result = String::from("First State");

        let result =
            Transition::<FirstState, FirstState>::transition_to_next_state(complex_state_machine)
                .expect("Should not fail the transitions to 'FirstState'.")
                .get_current_state()
                .get_state_name()
                .to_string();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_be_able_to_transition_multiple_times_state_when_transition_sequence_is_valid() {
        let complex_state_machine = ComplexStateMachine::new();

        let expected_result = String::from("Second State");

        let first_transition_result =
            Transition::<FirstState, FirstState>::transition_to_next_state(complex_state_machine)
                .expect("Should not fail the transitions to 'FirstState'.");
        let result = Transition::<FirstState, SecondState>::transition_to_next_state(
            first_transition_result,
        )
        .expect("Should not fail the transitions to 'SecondState'.")
        .get_current_state()
        .get_state_name()
        .to_string();

        assert_eq!(result, expected_result);
    }
}
