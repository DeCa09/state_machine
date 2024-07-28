use std::{fmt::Debug, hash::Hash};

pub trait ContextData:
    Debug + Send + Sync + Unpin + Clone + PartialEq + PartialOrd + Hash + Eq + Ord
{
    type UpdateType;

    fn get_context(&self) -> &Self;
    fn update_context(&mut self, updates: Self::UpdateType);
}
