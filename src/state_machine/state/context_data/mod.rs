use std::{fmt::Debug, hash::Hash};

pub trait ContextData:
    Debug + Send + Sync + Unpin + Clone + PartialEq + PartialOrd + Hash + Eq + Ord
{
    type UpdateType;

    fn get_context(&self) -> &Self;
    fn update_context(&mut self, updates: Self::UpdateType);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::common::{SampleStateContext, SampleStateContextUpdaterBuilder};

    #[test]
    fn should_return_reference_to_default_sample_context_when_initialized_with_default() {
        let sample_context = &SampleStateContext::default();

        let expected_result = &SampleStateContext::default();

        let result = sample_context.get_context();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_create_different_sample_context_with_custom_data_when_using_new_as_constructor() {
        let sample_context = &SampleStateContext::new(String::from("Demir ist der Boss."));

        let default_sample_context = &SampleStateContext::default();

        let result = sample_context.get_context();

        assert_ne!(result, default_sample_context);
    }

    #[test]
    fn should_update_context_data_to_random_string_with_update_context_method() {
        let mut context = SampleStateContext::default();
        let update = SampleStateContextUpdaterBuilder::default()
            .context_data(String::from("Updated Context!"))
            .build();

        let expected_result = &SampleStateContext::new(String::from("Updated Context!"));

        context.update_context(update);
        let result = context.get_context();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_leave_context_unchanged_when_empty_updates() {
        let mut context = SampleStateContext::default();
        let empty_update = SampleStateContextUpdaterBuilder::default().build();

        let expected_result = &SampleStateContext::default();

        context.update_context(empty_update);
        let result = context.get_context();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_leave_context_unchanged_when_using_default_updater() {
        let mut context = SampleStateContext::default();
        let empty_update = SampleStateContextUpdaterBuilder::default().build();

        let expected_result = &SampleStateContext::default();

        context.update_context(empty_update);
        let result = context.get_context();

        assert_eq!(result, expected_result);
    }
}
