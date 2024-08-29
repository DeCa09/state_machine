use crate::state_machine::state::State;

pub mod first_state_context;
pub mod first_state_data;

pub use first_state_context::{FirstStateContext, FirstStateContextUpdaterBuilder};
pub use first_state_data::{FirstStateData, FirstStateDataUpdaterBuilder};

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct FirstState {
    input: FirstStateData,
    output: Option<FirstStateData>,
    context_data: FirstStateContext,
}

impl State for FirstState {
    type InputData = FirstStateData;
    type OutputData = FirstStateData;
    type Context = FirstStateContext;

    fn get_state_name(&self) -> impl ToString {
        "First State"
    }

    fn get_input_data(&self) -> &FirstStateData {
        &self.input
    }

    fn compute_output_data(&mut self) {
        self.output = Some(FirstStateData::default());
    }

    fn get_output_data(&self) -> Option<&FirstStateData> {
        self.output.as_ref()
    }

    fn get_context_data(&self) -> &FirstStateContext {
        &self.context_data
    }
}
