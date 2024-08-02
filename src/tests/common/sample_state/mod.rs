use crate::state_machine::state::State;

pub mod sample_state_data;
pub mod sample_state_context;

pub use sample_state_data::SampleStateData;
pub use sample_state_context::{SampleStateContext, SampleStateContextUpdaterBuilder};

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct SampleState {
    input: SampleStateData,
    output: Option<SampleStateData>,
    context_data: SampleStateContext,
}

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

    fn get_context_data(&self) -> &SampleStateContext {
        &self.context_data
    }
}
