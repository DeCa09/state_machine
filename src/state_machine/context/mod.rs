use std::{fmt::Debug, hash::Hash};

pub trait ContextData:
    Debug + Send + Sync + Unpin + Clone + PartialEq + PartialOrd + Hash + Eq + Ord
{
    // Add any common methods that all shared data should implement, if needed
}
