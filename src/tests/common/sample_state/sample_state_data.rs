use crate::state_machine::state::StateData;

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct SampleStateData{
    state_data: String,
}


impl SampleStateData {
    pub fn new(state_data: String) -> Self {
        Self { state_data }
    }
}

impl StateData for SampleStateData {}
