//! Spatial lookup algorithms
//!
//! You can implement your own algorithm by implementing the `SpatialLookupAlgorithm` trait.

mod bvh;
mod naive;

// Re-export algorithms for ease of use.
pub use bvh::Bvh;
pub use naive::Naive;

/// Common tests which test all algorithms with the same World setup,
/// to make sure they all return the same entities.
///
/// TODO: Consider using a fixture-based test framework
#[cfg(test)]
mod tests {
    use crate::{SpatialLookupState, algorithms};
    use bevy::prelude::*;
    use turborand::SeededCore;
    use turborand::prelude::*;

    const WORLD_SIZE: f32 = 10.0;
    const LOOKUP_RADIUS: f32 = 1.0;

    /// Helper function to make a list of pseudo-randomly places entities
    fn world_with_n_entities(n: u32) -> Vec<(Entity, Vec3)> {
        let rng = Rng::with_seed(417311532);
        let mut entities = Vec::with_capacity(n as usize);

        for i in 0..n {
            entities.push((
                Entity::from_raw(i),
                Vec3::new(
                    rng.f32_normalized() * WORLD_SIZE,
                    rng.f32_normalized() * WORLD_SIZE,
                    rng.f32_normalized() * WORLD_SIZE,
                ),
            ));
        }

        entities
    }

    #[test]
    fn test_bvh_in_range() {
        let mut lookup_state = SpatialLookupState::with_algorithm(algorithms::Bvh::default());
        lookup_state.entities = world_with_n_entities(100_000);
        lookup_state.prepare_algorithm();

        let found = lookup_state.entities_in_radius(Vec3::ZERO, LOOKUP_RADIUS);

        assert_eq!(found.len(), 39);
    }

    #[test]
    fn test_naive_in_range() {
        let mut lookup_state = SpatialLookupState::with_algorithm(algorithms::Naive::default());
        lookup_state.entities = world_with_n_entities(100_000);
        lookup_state.prepare_algorithm();

        let found = lookup_state.entities_in_radius(Vec3::ZERO, LOOKUP_RADIUS);

        assert_eq!(found.len(), 39);
    }
}
