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

impl SampleStateMachine {
    pub fn new() -> Self {
        SampleStateMachine::default()
    }
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

impl State for SampleStateMachine {
    type InputData = SampleStateData;
    type OutputData = SampleStateData;
    type Context = SampleStateContext;

    fn get_state_name(&self) -> impl ToString {
        format!(
            "State Machine with current state: {}",
            self.current_state.get_state_name().to_string()
        )
    }

    fn get_input_data(&self) -> &Self::InputData {
        &self.input
    }

    fn compute_output_data(&mut self) {
        self.current_state.compute_output_data();
        self.output = self.current_state.get_output_data().cloned();
    }

    fn get_output_data(&self) -> Option<&Self::OutputData> {
        self.current_state.get_output_data()
    }

    fn get_context_data(&self) -> &Self::Context {
        &self.context_data
    }
}
