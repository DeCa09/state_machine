use std::{fmt::Debug, hash::Hash};

pub trait StateData:
    Debug + Send + Sync + Unpin + Clone + PartialEq + PartialOrd + Hash + Eq + Ord
{
    type UpdateType;

    fn get_state(&self) -> &Self;
    fn update_state(&mut self, updates: Self::UpdateType);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::common::{SampleStateData, SampleStateDataUpdaterBuilder};

    #[test]
    fn should_return_reference_to_default_sample_state_data_when_initialized_with_default() {
        let sample_state_data = &SampleStateData::default();

        let expected_result = &SampleStateData::default();

        let result = sample_state_data.get_state();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_create_different_state_data_with_custom_data_when_using_new_as_constructor() {
        let sample_state_data = &SampleStateData::new(String::from("Demir ist der Boss."));

        let default_sample_state_data = &SampleStateData::default();

        let result = sample_state_data.get_state();

        assert_ne!(result, default_sample_state_data);
    }

    #[test]
    fn should_update_state_data_to_specified_string_when_update_contains_specified_string() {
        let mut state_data = SampleStateData::default();
        let update = SampleStateDataUpdaterBuilder::default()
            .state_data(String::from("Updated State!"))
            .build();

        let expected_result = &SampleStateData::new(String::from("Updated State!"));

        state_data.update_state(update);
        let result = state_data.get_state();

        assert_eq!(result, expected_result);
    }
}
