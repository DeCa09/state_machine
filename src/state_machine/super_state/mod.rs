use super::{state::State, StateMachine};

pub trait SuperState<S: State>: StateMachine<S> + State {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state_machine::transition::Transition;
    use crate::tests::common::HierarchicalStateMachine;

    #[test]
    fn should_transition_to_second_inner_state_when_in_first_inner_state_of_sample_super_state() {
        let hierarchical_state_machine = HierarchicalStateMachine::new();

        let expected_result = String::from("Second Inner State");

        let result = hierarchical_state_machine
            .transition_to_next_state()
            .expect("Should not fail inner super state transition to 'SecondInnerState'")
            .get_current_state()
            .get_current_state()
            .get_state_name()
            .to_string();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_transition_to_outer_sample_state_when_in_second_inner_state_of_sample_super_state() {
        let hierarchical_state_machine = HierarchicalStateMachine::new();

        let expected_result = String::from("Sample State");

        let hierarchical_state_machine = hierarchical_state_machine
            .transition_to_next_state()
            .expect("Should not fail inner super state transition to 'SecondInnerState");
        let result = hierarchical_state_machine.transition_to_next_state()
        .expect("Should not fail transition from inner state 'SecondInnerState' to outer state 'SampleState'")
        .get_current_state()
        .get_state_name()
        .to_string();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_super_state_name_when_calling_get_state_name_from_super_state() {
        let hierarchical_state_machine = HierarchicalStateMachine::new();

        let expected_result = String::from("Super State");

        let result = hierarchical_state_machine
            .get_current_state()
            .get_state_name()
            .to_string();

        assert_eq!(result, expected_result);
    }
}
