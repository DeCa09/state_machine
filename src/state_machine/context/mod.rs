use std::fmt::Debug;

pub trait ContextData: Debug + PartialEq + Eq {
    // Add any common methods that all shared data should implement, if needed
}
