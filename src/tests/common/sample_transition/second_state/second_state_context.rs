use crate::state_machine::state::ContextData;

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct SecondStateContext {
    context_data: String,
}

impl SecondStateContext {
    pub fn new(context_data: String) -> Self {
        Self { context_data }
    }
}

impl ContextData for SecondStateContext {
    type UpdateType = SecondStateContextUpdater;
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
pub struct SecondStateContextUpdater {
    pub context_data: Option<String>,
}

pub struct SecondStateContextUpdaterBuilder {
    context_data: Option<String>,
}

impl SecondStateContextUpdaterBuilder {
    pub fn new() -> Self {
        Self { context_data: None }
    }

    pub fn context_data(mut self, context_data: String) -> Self {
        self.context_data = Some(context_data);
        self
    }

    pub fn build(self) -> SecondStateContextUpdater {
        SecondStateContextUpdater {
            context_data: self.context_data,
        }
    }
}

impl Default for SecondStateContextUpdaterBuilder {
    fn default() -> Self {
        Self::new()
    }
}
