use std::{fmt::Debug, hash::Hash};
pub trait StateData:
    Debug + Send + Sync + Unpin + Clone + PartialEq + PartialOrd + Hash + Eq + Ord
{
}
