use crate::state_machine::state::{ContextData, State, StateData};

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct SampleState {
    input: SampleStateData,
    output: Option<SampleStateData>,
    context_data: SampleStateContext,
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Hash, Eq, Ord)]

pub struct SampleStateData;

impl StateData for SampleStateData {}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct SampleStateContext;

impl ContextData for SampleStateContext {}

impl State for SampleState {
    type InputData = SampleStateData;
    type OutputData = SampleStateData;
    type Context = SampleStateContext;

    fn get_state_name(&self) -> String {
        "Sample State".to_string()
    }

    fn get_input_data(&self) -> &SampleStateData {
        &self.input
    }

    fn compute_output_data(&mut self) {
        self.output = Some(SampleStateData::default());
    }

    fn get_output_data(&self) -> Option<&SampleStateData> {
        self.output.as_ref()
    }

    fn get_context_data(&self) -> SampleStateContext {
        self.context_data.clone()
    }
}
