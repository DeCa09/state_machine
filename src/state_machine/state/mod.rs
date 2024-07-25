use std::{fmt::Debug, hash::Hash};

pub mod context_data;
pub mod state_data;


pub use context_data::ContextData;
pub use state_data::StateData;
pub trait State:
    Debug + Send + Sync + Unpin + Clone + PartialEq + PartialOrd + Hash + Eq + Ord
{
    type InputData: StateData;
    type OutputData: StateData;
    type Context: ContextData;

    fn get_state_name(&self) -> String;

    fn get_input_data(&self) -> &Self::InputData;

    fn compute_output_data(&mut self);

    fn get_output_data(&self) -> Option<&Self::OutputData>;

    fn has_output_data_been_computed(&self) -> bool {
        self.get_output_data().is_some()
    }

    fn get_context_data(&self) -> Self::Context;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::common::{SampleState, SampleStateContext, SampleStateData};

    #[test]
    fn should_return_name_of_sample_state_when_in_sample_state() {
        let sample_state = SampleState::default();

        let expected_result = String::from("Sample State");

        let result = sample_state.get_state_name();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_default_state_data_struct_as_input_data_when_output_data_has_not_been_computed_in_state(
    ) {
        let sample_state = SampleState::default();

        let expected_result = &SampleStateData::default();

        let result = sample_state.get_input_data();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_default_state_data_struct_as_input_data_when_in_initial_sample_state() {
        let sample_state = SampleState::default();

        let expected_result = &SampleStateData::default();

        let result = sample_state.get_input_data();

        assert_eq!(result, expected_result);
    }

    #[test]
    #[should_panic]
    fn should_panic_when_trying_to_access_output_data_before_it_has_been_computed_in_state() {
        let sample_state = SampleState::default();

        let expected_result = &SampleStateData::default();

        let result = sample_state
            .get_output_data()
            .expect("The output should be a non-empty default 'SampleStateData' struct.");

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_false_when_state_has_not_computed_the_output() {
        let sample_state = SampleState::default();

        let expected_result = false;

        let result = sample_state.has_output_data_been_computed();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_true_when_state_has_computed_the_output() {
        let mut sample_state = SampleState::default();

        let expected_result = true;

        sample_state.compute_output_data();
        let result = sample_state.has_output_data_been_computed();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_default_context_data_when_in_initial_state() {
        let sample_state = SampleState::default();

        let expected_result = SampleStateContext::default();

        let result = sample_state.get_context_data();

        assert_eq!(result, expected_result);
    }

    fn implements_auto_traits<T: Sized + Send + Sync + Unpin>() {}
    #[test]
    fn should_still_implement_auto_traits_traits_when_implementing_state_trait() {
        implements_auto_traits::<SampleState>();
    }

    fn implements_send<T: Send>() {}
    fn implements_sync<T: Sync>() {}

    #[test]
    fn should_implement_send_when_implementing_state_trait() {
        implements_send::<SampleState>();
    }

    #[test]
    fn should_implement_sync_when_implementing_state_trait() {
        implements_sync::<SampleState>();
    }

    #[test]
    fn should_be_thread_safe_when_implementing_state_trait() {
        implements_send::<SampleState>();
        implements_sync::<SampleState>();
    }

    fn implements_sized<T: Sized>() {}
    #[test]
    fn should_be_sized_when_implementing_state_trait() {
        implements_sized::<SampleState>();
    }

    fn implements_hash<T: Hash>() {}
    #[test]
    fn should_implement_hash_when_implementing_state_trait() {
        implements_hash::<SampleState>();
    }

    fn implements_partial_eq<T: PartialEq>() {}
    #[test]
    fn should_implement_partial_eq_when_implementing_state_trait() {
        implements_partial_eq::<SampleState>();
    }

    fn implements_eq<T: Eq>() {}
    #[test]
    fn should_implement_eq_when_implementing_state_trait() {
        implements_eq::<SampleState>();
    }

    fn implements_partial_ord<T: PartialOrd>() {}
    #[test]
    fn should_implement_partial_ord_when_implementing_state_trait() {
        implements_partial_ord::<SampleState>();
    }

    fn implements_ord<T: Ord>() {}
    #[test]
    fn should_implement_ord_when_implementing_state_trait() {
        implements_ord::<SampleState>();
    }

    fn implements_default<T: Default>() {}
    #[test]
    fn should_implement_default_when_implementing_state_trait() {
        implements_default::<SampleState>()
    }

    fn implements_debug<T: Debug>() {}
    #[test]
    fn should_implement_debug_when_implementing_state_trait() {
        implements_debug::<SampleState>();
    }

    fn implements_clone<T: Clone>() {}
    #[test]
    fn should_implement_clone_when_implementing_state_trait() {
        implements_clone::<SampleState>();
    }

    fn implements_unpin<T: Unpin>() {}
    #[test]
    fn should_implement_unpin_when_implementing_state_trait() {
        implements_unpin::<SampleState>();
    }

    #[test]
    fn should_return_default_context_data_when_called_with_state_reference() {
        let sample_state = &SampleState::default();
        let ref_to_sample_state = &SampleState::default();

        let expected_result = sample_state.get_context_data();

        let result = ref_to_sample_state.get_context_data();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_true_when_reference_state_has_computed_the_output() {
        let ref_to_sample_state = &mut SampleState::default();

        let expected_result = true;

        ref_to_sample_state.compute_output_data();
        let result = ref_to_sample_state.has_output_data_been_computed();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_false_when_reference_state_has_not_computed_the_output() {
        let ref_to_sample_state = &mut SampleState::default();

        let expected_result = false;

        let result = ref_to_sample_state.has_output_data_been_computed();

        assert_eq!(result, expected_result);
    }

    #[test]
    #[should_panic]
    fn should_panic_when_trying_to_access_output_data_before_it_has_been_computed_in_reference_state(
    ) {
        let ref_to_sample_state = &SampleState::default();

        let expected_result = &SampleStateData::default();

        let result = ref_to_sample_state
            .get_output_data()
            .expect("The output should be a non-empty default 'SampleStateData' struct.");

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_name_of_sample_state_when_calling_reference_to_sample_state() {
        let ref_to_sample_state = &SampleState::default();

        let expected_result = String::from("Sample State");

        let result = ref_to_sample_state.get_state_name();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_default_state_data_as_input_data_when_output_data_has_not_been_computed_in_reference_state(
    ) {
        let ref_to_sample_state = &SampleState::default();

        let expected_result = &SampleStateData::default();

        let result = ref_to_sample_state.get_input_data();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_default_state_data_as_input_data_when_reference_sample_state_in_initial_state()
    {
        let ref_to_sample_state = &SampleState::default();

        let expected_result = &SampleStateData::default();

        let result = ref_to_sample_state.get_input_data();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_not_change_input_data_when_computing_output_data() {
        let mut sample_state = SampleState::default();

        let expected_result = &sample_state.get_input_data().clone();

        sample_state.compute_output_data();
        let result = sample_state.get_input_data();

        assert_eq!(result, expected_result)
    }
}
