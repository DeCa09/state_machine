use crate::state_machine::state::ContextData;
use crate::state_machine::{state::State, StateMachine};

pub trait Transition<C: ContextData, S: State> {
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
    fn transition_to_next_state(self) -> Result<StateMachine<C, Self::NextState>, &'static str>;
    //fn transition_to_next_state(self) -> Result<StateMachine<Self::NextState>, Self::Error>;
}
