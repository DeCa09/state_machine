use crate::state_machine::context::ContextData;
use crate::state_machine::{state::State, StateMachine};

pub trait Transition<C: ContextData, S: State> {
    type NextState: State;

    //type Error;

    fn transition_to_next_state(self) -> Result<StateMachine<C, Self::NextState>, &'static str>;
    //fn transition_to_next_state(self) -> Result<StateMachine<Self::NextState>, Self::Error>;
}
