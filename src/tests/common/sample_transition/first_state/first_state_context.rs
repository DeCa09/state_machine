use crate::state_machine::state::ContextData;

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct FirstStateContext {
    context_data: String,
}

impl FirstStateContext {
    pub fn new(context_data: String) -> Self {
        Self { context_data }
    }
}

impl ContextData for FirstStateContext {
    type UpdateType = FirstStateContextUpdater;
    fn get_context(&self) -> &Self {
        self
    }

    fn update_context(&mut self, updates: Self::UpdateType) {
        if let Some(value) = updates.context_data {
            self.context_data = value;
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct FirstStateContextUpdater {
    pub context_data: Option<String>,
}

pub struct FirstStateContextUpdaterBuilder {
    context_data: Option<String>,
}

impl FirstStateContextUpdaterBuilder {
    pub fn new() -> Self {
        Self { context_data: None }
    }

    pub fn context_data(mut self, context_data: String) -> Self {
        self.context_data = Some(context_data);
        self
    }

    pub fn build(self) -> FirstStateContextUpdater {
        FirstStateContextUpdater {
            context_data: self.context_data,
        }
    }
}

impl Default for FirstStateContextUpdaterBuilder {
    fn default() -> Self {
        Self::new()
    }
}
