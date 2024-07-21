use super::context::ContextData;
use data::StateData;
use std::{fmt::Debug, hash::Hash};

pub mod data;

pub trait State:
    Debug + Send + Sync + Unpin + Clone + PartialEq + PartialOrd + Hash + Eq + Ord
{
    type InputData: StateData;
    type OutputData: StateData;
    type Context: ContextData;

    fn get_state_name(&self) -> String;

    fn get_input_data(&self) -> Self::InputData;
    fn compute_output_data(&mut self);

    fn get_output_data(&self) -> Option<Self::OutputData>;
    fn has_output_data_been_computed(&self) -> bool;

    fn get_context_data(&self) -> Self::Context;
}

#[cfg(test)]
mod tests {
    use std::panic::UnwindSafe;

    use super::*;
    use crate::tests::common::{StartingState, StartingStateContext, StartingStateData};

    #[test]
    fn should_return_name_of_starting_state_when_in_starting_state() {
        let starting_state = StartingState::default();

        let expected_result = String::from("Starting State");

        let result = starting_state.get_state_name();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_default_custom_state_data_struct_as_input_data_when_output_data_has_not_been_computed_in_initial_starting_state(
    ) {
        let starting_state = StartingState::default();

        let expected_result = StartingStateData::default();

        let result = starting_state.get_input_data();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_default_custom_state_data_struct_as_input_data_when_in_initial_starting_state()
    {
        let starting_state = StartingState::default();

        let expected_result = StartingStateData::default();

        let result = starting_state.get_input_data();

        assert_eq!(result, expected_result);
    }

    #[test]
    #[should_panic]
    fn should_panic_when_trying_to_access_output_data_before_it_has_been_computed_in_initial_starting_state(
    ) {
        let starting_state = StartingState::default();

        let expected_result = StartingStateData::default();

        let result = starting_state
            .get_output_data()
            .expect("The output should be a non-empty default 'StartingStateData' struct.");

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_false_when_starting_state_has_not_computed_the_output() {
        let starting_state = StartingState::default();

        let expected_result = false;

        let result = starting_state.has_output_data_been_computed();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_true_when_starting_state_has_computed_the_output() {
        let mut starting_state = StartingState::default();

        let expected_result = true;

        starting_state.compute_output_data();
        let result = starting_state.has_output_data_been_computed();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_default_custom_context_struct_as_context_data_when_in_starting_state() {
        let starting_state = StartingState::default();

        let expected_result = StartingStateContext::default();

        let result = starting_state.get_context_data();

        assert_eq!(result, expected_result);
    }

    fn implements_auto_traits<T: Sized + Send + Sync + Unpin + UnwindSafe>() {}
    #[test]
    fn should_still_implement_auto_traits_traits_when_implementing_state_trait() {
        implements_auto_traits::<StartingState>();
    }

    fn implements_send<T: Send>() {}
    fn implements_sync<T: Sync>() {}

    #[test]
    fn should_implement_send_when_implementing_state_trait() {
        implements_send::<StartingState>();
    }

    #[test]
    fn should_implement_sync_when_implementing_state_trait() {
        implements_sync::<StartingState>();
    }

    #[test]
    fn should_be_thread_safe_when_implementing_state_trait() {
        implements_send::<StartingState>();
        implements_sync::<StartingState>();
    }

    fn implements_sized<T: Sized>() {}
    #[test]
    fn should_be_sized_when_implementing_state_trait() {
        implements_sized::<StartingState>();
    }

    fn implements_hash<T: Hash>() {}
    #[test]
    fn should_be_hashable_when_implementing_state_trait() {
        implements_hash::<StartingState>();
    }

    fn implements_partial_eq<T: PartialEq>() {}
    #[test]
    fn should_implement_partial_eq_when_implementing_state_trait() {
        implements_partial_eq::<StartingState>();
    }

    fn implements_eq<T: Eq>() {}
    #[test]
    fn should_implement_eq_when_implementing_state_trait() {
        implements_eq::<StartingState>();
    }

    fn implements_partial_ord<T: PartialOrd>() {}
    #[test]
    fn should_implement_partial_ord_when_implementing_state_trait() {
        implements_partial_ord::<StartingState>();
    }

    fn implements_ord<T: Ord>() {}
    #[test]
    fn should_implement_ord_when_implementing_state_trait() {
        implements_ord::<StartingState>();
    }

    fn implements_default<T: Default>() {}
    #[test]
    fn should_implement_default_when_implementing_state_trait() {
        implements_default::<StartingState>()
    }

    fn implements_debug<T: Debug>() {}
    #[test]
    fn should_implement_debug_when_implementing_state_trait() {
        implements_debug::<StartingState>();
    }

    fn implements_clone<T: Clone>() {}
    #[test]
    fn should_implement_clone_when_implementing_state_trait() {
        implements_clone::<StartingState>();
    }
}
