//! Spatially aware Queries for the Bevy game engine

use bevy::prelude::*;

mod algorithms;
mod spatial_query;
mod spatial_query_iterator;

pub mod prelude {
    pub use crate::SpatialQueriesPlugin;
    pub use crate::spatial_query::SpatialQuery;
    pub use crate::spatial_query_iterator::SpatialQueryIterator;
}

pub struct SpatialQueriesPlugin;

impl Plugin for SpatialQueriesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpatialLookupState::default())
            .add_systems(First, build_spatial_lookup);
    }
}

pub trait SpatialLookupAlgorithm {
    /// Prepares the lookup algorithm with a fresh set of entities and their positions.
    fn prepare(&mut self, entities: &[(Entity, Vec3)]);

    /// Returns a list of all entities that are within the given radius of the sample point.
    fn entities_in_radius(&self, sample_point: Vec3, radius: f32) -> Vec<Entity>;
}

#[derive(Resource)]
pub struct SpatialLookupState {
    entities: Vec<(Entity, Vec3)>,
    algorithm: Box<dyn SpatialLookupAlgorithm + Send + Sync>,
}

impl Default for SpatialLookupState {
    fn default() -> Self {
        SpatialLookupState {
            entities: Vec::new(),
            algorithm: Box::new(algorithms::Bvh::default()),
        }
    }
}

impl SpatialLookupState {
    pub fn entities_in_radius(&self, sample_point: Vec3, radius: f32) -> Vec<Entity> {
        self.algorithm.entities_in_radius(sample_point, radius)
    }

    pub fn prepare_algorithm(&mut self) {
        self.algorithm.prepare(&self.entities);
    }
}

fn build_spatial_lookup(
    all_entities: Query<(Entity, &GlobalTransform)>,
    mut lookup_state: ResMut<SpatialLookupState>,
) {
    lookup_state.entities.clear();

    for (entity, transform) in &all_entities {
        lookup_state
            .entities
            .push((entity.clone(), transform.translation()));
    }

    lookup_state.prepare_algorithm();
}
