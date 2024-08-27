pub mod state;
pub mod transition;

use crate::state_machine::state::State;

pub trait StateMachine<S: State>: State {
    fn get_current_state(&self) -> impl ToString {
        self.get_state_name()
    }

    fn run(&mut self);

    fn advance_state(&mut self);
}
