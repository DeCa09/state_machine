use crate::state_machine::state::ContextData;

pub struct SampleStateContextUpdater{
    context_data: Option<String>
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct SampleStateContext{
    context_data: String,
}

impl ContextData for SampleStateContext {
    type UpdateType = SampleStateContextUpdater;
    fn get_context(&self) -> &Self {
        self
    }

    fn update_context(&mut self, updates: Self::UpdateType) {
        if let Some(value) = updates.context_data {
            self.context_data = value;
        }
    }

}