use std::fmt::Debug;

use crate::state_machine::{state::State, StateMachine};
use crate::tests::common::{SampleState, SampleStateContext, SampleStateData};

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct SampleStateMachine {
    current_state: SampleState,
    input: SampleStateData,
    output: Option<SampleStateData>,
    context_data: SampleStateContext,
}

impl StateMachine<SampleState> for SampleStateMachine {
    fn get_current_state(&self) -> &SampleState {
        &self.current_state
    }

    fn run(&mut self) {
        // Example implementation of run method
        // In a real scenario, this would contain logic to perform operations based on the current state
        println!(
            "Running state: {}",
            self.get_current_state().get_state_name().to_string()
        );
    }

    fn advance_state(&mut self) {
        // Example implementation to change state
        // Here we could implement logic to transition to another state
        println!("Advancing state");
        // Example: Simulate state change by computing output data
        self.current_state.compute_output_data();
    }
}
