use crate::state_machine::context::ContextData;
use crate::state_machine::state::{data::StateData, State};

#[derive(Debug)]
pub struct StartingState {
    input: StartingStateData,
    output: Option<StartingStateData>,
    context_data: StartingStateContext,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StartingStateData;

impl StateData for StartingStateData {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StartingStateContext;

impl ContextData for StartingStateContext {}

impl State for StartingState {
    type InputData = StartingStateData;
    type OutputData = StartingStateData;
    type Context = StartingStateContext;

    fn get_state_name(&self) -> String {
        "Starting State".to_string()
    }

    fn get_input_data(&self) -> StartingStateData {
        self.input
    }

    fn compute_output_data(&mut self) {
        self.output = Some(StartingStateData::default());
    }

    fn get_output_data(&self) -> Option<StartingStateData> {
        self.output
    }

    fn has_output_data_been_computed(&self) -> bool {
        self.get_output_data().is_some()
    }

    fn get_context_data(&self) -> StartingStateContext {
        self.context_data
    }
}

impl Default for StartingState {
    fn default() -> Self {
        StartingState {
            input: StartingStateData::default(),
            output: None,
            context_data: StartingStateContext::default(),
        }
    }
}
