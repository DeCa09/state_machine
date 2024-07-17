use crate::state_machine::state::State;

#[derive(Debug)]
pub struct StartingState;

impl State for StartingState {
    fn get_state_name(self) -> String {
        "Starting State".to_string()
    }
}
