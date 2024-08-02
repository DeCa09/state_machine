use std::{fmt::Debug, hash::Hash};

pub trait StateData:
    Debug + Send + Sync + Unpin + Clone + PartialEq + PartialOrd + Hash + Eq + Ord
{
    type UpdateType;

    fn get_state(&self) -> &Self;
    fn update_state(&mut self, updates: Self::UpdateType);
}
