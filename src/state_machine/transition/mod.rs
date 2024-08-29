use crate::state_machine::{state::State, StateMachine};

pub trait Transition<S: State>: StateMachine<S> {
    type NextState: State;

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
    fn transition_to_next_state(self) -> Result<Self::NextState, &'static str>;
    //fn transition_to_next_state(self) -> Result<StateMachine<Self::NextState>, Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state_machine::transition::Transition;
    use crate::tests::common::ComplexStateMachine;

    #[test]
    fn should_transition_to_second_state_when_in_first_state() {
        let complex_state_machine = ComplexStateMachine::new();

        let expected_result = String::from("Second State");

        let result = complex_state_machine
            .transition_to_next_state()
            .expect("Should not fail the transitions to 'SecondState'.")
            .get_state_name()
            .to_string();

        assert_eq!(result, expected_result);
    }
}
