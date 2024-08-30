use crate::state_machine::{state::State, transition::Transition, StateMachine};

use super::{first_state::FirstState, second_state::SecondState};

// Define the StateMachine that uses FirstState and SecondState
#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct ComplexStateMachine<S: State> {
    current_state: S,
}

impl<S: State> ComplexStateMachine<S> {
    fn new_machine(self) -> Result<ComplexStateMachine<SecondState>, &'static str> {
        Ok(ComplexStateMachine {
            current_state: SecondState::default(),
        })
    }
}

impl ComplexStateMachine<FirstState> {
    pub fn new() -> Self {
        Self {
            current_state: FirstState::default(),
        }
    }
}

impl StateMachine<FirstState> for ComplexStateMachine<FirstState> {
    fn get_current_state(&self) -> &FirstState {
        &self.current_state
    }

    fn get_current_state_mut(&mut self) -> &mut FirstState {
        &mut self.current_state
    }

    fn run(&mut self) {
        println!(
            "Running state: {}",
            self.get_current_state().get_state_name().to_string()
        );
    }

    fn advance_state(&mut self) {
        println!("Advancing state");
        self.get_current_state_mut().compute_output_data();
    }
}

impl StateMachine<SecondState> for ComplexStateMachine<SecondState> {
    fn get_current_state(&self) -> &SecondState {
        &self.current_state
    }

    fn get_current_state_mut(&mut self) -> &mut SecondState {
        &mut self.current_state
    }

    fn run(&mut self) {
        println!(
            "Running state: {}",
            self.get_current_state().get_state_name().to_string()
        );
    }

    fn advance_state(&mut self) {
        println!("Advancing state");
        self.get_current_state_mut().compute_output_data();
    }
}

impl Transition<FirstState, FirstState> for ComplexStateMachine<FirstState> {
    type NextState = FirstState;
    type NextStateMachine = ComplexStateMachine<Self::NextState>;

    fn transition_to_next_state(self) -> Result<Self::NextStateMachine, &'static str> {
        Ok(ComplexStateMachine {
            current_state: FirstState::default(),
        })
    }
}

impl Transition<FirstState, SecondState> for ComplexStateMachine<FirstState> {
    type NextState = SecondState;
    type NextStateMachine = ComplexStateMachine<Self::NextState>;

    fn transition_to_next_state(self) -> Result<Self::NextStateMachine, &'static str> {
        Ok(ComplexStateMachine {
            current_state: SecondState::default(),
        })
    }
}
