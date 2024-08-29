use crate::state_machine::state::StateData;

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct FirstStateData {
    state_data: String,
}

impl FirstStateData {
    pub fn new(state_data: String) -> Self {
        Self { state_data }
    }
}

impl StateData for FirstStateData {
    type UpdateType = FirstStateDataUpdater;
    fn get_state(&self) -> &Self {
        self
    }

    fn update_state(&mut self, updates: Self::UpdateType) {
        if let Some(value) = updates.state_data {
            self.state_data = value;
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct FirstStateDataUpdater {
    pub state_data: Option<String>,
}

pub struct FirstStateDataUpdaterBuilder {
    state_data: Option<String>,
}

impl FirstStateDataUpdaterBuilder {
    pub fn new() -> Self {
        Self { state_data: None }
    }

    pub fn state_data(mut self, state_data: String) -> Self {
        self.state_data = Some(state_data);
        self
    }

    pub fn build(self) -> FirstStateDataUpdater {
        FirstStateDataUpdater {
            state_data: self.state_data,
        }
    }
}

impl Default for FirstStateDataUpdaterBuilder {
    fn default() -> Self {
        Self::new()
    }
}
