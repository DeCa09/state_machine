use std::fmt::Debug;

pub mod data;

pub trait State: Debug {
    fn get_state_name(self) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::common::StartingState;

    const STARTING_STATE: StartingState = StartingState;

    #[test]
    fn test_starting_state_name() {
        let expected_result = String::from("Starting State");

        let result = STARTING_STATE.get_state_name();

        assert_eq!(result, expected_result);
    }
}
