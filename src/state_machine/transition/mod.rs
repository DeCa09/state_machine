use crate::state_machine::{state::State, StateMachine};

/// A trait that defines the ability of a state machine to transition from one state to another.
///
/// The `Transition` trait extends the `StateMachine` trait, allowing a state machine to move from
/// one state of type `T` to a new state of type `U`. This trait is generic over the states `T` and `U`,
/// which must both implement the `State` trait. The trait also requires the current state machine to
/// implement `StateMachine<T>`.
///
/// # Associated Types
///
/// - `NewStateMachine`: Represents the type of the state machine after the transition has been performed.
///   This type must implement the `StateMachine<U>` trait, ensuring the state machine is valid in the new state.
///
/// # Errors
///
/// Implementors of this trait must handle potential errors that could occur during state transitions.
/// These errors can occur due to several reasons:
/// - Attempting to transition to a state that is not defined in the state machine.
/// - Incompatibilities between the data formats of the current state and the new state.
/// - Logical errors in the state transition logic, such as invalid conditions for transitioning.
///
pub trait Transition<T: State, U: State>: StateMachine<T> {
    /// The type of the state machine after transitioning to the new state.
    ///
    /// This type represents the state machine in its new state. It must implement the `StateMachine<U>` trait,
    /// which ensures that it correctly represents a state machine capable of handling the state `U`.
    type NewStateMachine: StateMachine<U>;
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

    fn transition_to_next_state(self) -> Result<Self::NewStateMachine, &'static str>;
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
