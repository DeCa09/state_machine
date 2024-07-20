use crate::state_machine::context::ContextData;
use crate::state_machine::state::{data::StateData, State};

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct StartingState {
    input: StartingStateData,
    output: Option<StartingStateData>,
    context_data: StartingStateContext,
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Hash, Eq, Ord)]

pub struct StartingStateData;

impl StateData for StartingStateData {}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Hash, Eq, Ord)]
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
        self.input.clone()
    }

    fn compute_output_data(&mut self) {
        self.output = Some(StartingStateData::default());
    }

    fn get_output_data(&self) -> Option<StartingStateData> {
        self.output.clone()
    }

    fn has_output_data_been_computed(&self) -> bool {
        self.get_output_data().is_some()
    }

    fn get_context_data(&self) -> StartingStateContext {
        self.context_data.clone()
    }
}
