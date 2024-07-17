pub mod context;
pub mod state;
pub mod transition;

use crate::state_machine::context::ContextData;
use crate::state_machine::state::State;

#[derive(Debug)]
pub struct StateMachine<C, S>
where
    C: ContextData,
    S: State,
{
    pub context_data: C,
    pub state: S,
}
