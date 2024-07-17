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
    fn should_return_name_of_starting_state_when_in_starting_state() {
        let starting_state = STARTING_STATE;

        let expected_result = String::from("Starting State");

        let result = starting_state.get_state_name();

        assert_eq!(result, expected_result);
    }
}
