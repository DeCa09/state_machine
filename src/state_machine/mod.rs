pub mod state;
pub mod transition;

use crate::state_machine::state::State;

#[derive(Debug)]
pub struct StateMachine<S>
where
    S: State,
{
    pub state: S,
}
