use agave_feature_set::{FeatureSet, FEATURE_NAMES};

fn main() {
    let mut feature_manager = FeatureSet::default();

    // Activate SIMD 256 from slot zero
    feature_manager.activate(&agave_feature_set::raise_block_limits_to_60m::id(), 0);

    // To deactivate a feature, example  SIMD 256
    feature_manager.deactivate(&agave_feature_set::raise_block_limits_to_60m::id());

    // To check whether a feature is active, example  SIMD 256
    feature_manager.is_active(&agave_feature_set::raise_block_limits_to_60m::id());

    // To check the slot when a certain feature was activated, example  SIMD 256
    feature_manager.activated_slot(&agave_feature_set::raise_block_limits_to_60m::id());

    // To get all active features
    feature_manager.active();

    // To get all inactive features
    feature_manager.inactive();

    // To get all enabled features that trigger full inflation
    feature_manager.full_inflation_features_enabled();

    // To instantiate a `FeatureSet` struct with all features enabled at once
    let feature_manager = FeatureSet::all_enabled();

    for (feature_public_key, feature_description) in FEATURE_NAMES.iter() {
        println!("{feature_public_key} - {feature_description}")
    }
}
