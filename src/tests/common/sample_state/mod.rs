use std::fmt::Debug;

use crate::state_machine::state::State;

pub mod sample_state_context;
pub mod sample_state_data;

pub use sample_state_context::{SampleStateContext, SampleStateContextUpdaterBuilder};
pub use sample_state_data::{SampleStateData, SampleStateDataUpdaterBuilder};

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

    fn get_state_name(&self) -> impl ToString {
        "Sample State"
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
