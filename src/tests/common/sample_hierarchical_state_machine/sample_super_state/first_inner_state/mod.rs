use crate::state_machine::state::State;

pub mod first_inner_state_context;
pub mod first_inner_state_data;

pub use first_inner_state_context::FirstInnerStateContext;
pub use first_inner_state_data::FirstInnerStateData;

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct FirstInnerState {
    input: FirstInnerStateData,
    output: Option<FirstInnerStateData>,
    context_data: FirstInnerStateContext,
}

impl State for FirstInnerState {
    type InputData = FirstInnerStateData;
    type OutputData = FirstInnerStateData;
    type Context = FirstInnerStateContext;

    fn get_state_name(&self) -> impl ToString {
        "First State"
    }

    fn get_input_data(&self) -> &FirstInnerStateData {
        &self.input
    }

    fn compute_output_data(&mut self) {
        self.output = Some(FirstInnerStateData::default());
    }

    fn get_output_data(&self) -> Option<&FirstInnerStateData> {
        self.output.as_ref()
    }

    fn get_context_data(&self) -> &FirstInnerStateContext {
        &self.context_data
    }
}
