//! Naive Spatial Lookup: Just iterate all entities every time!
use crate::prelude::*;
use bevy::prelude::*;

/// Naive spatial lookup: just iterate all entities every time.
///
/// This "algorithm" will outperfom BVH in cases where there is
/// Only one lookup per rebuild (entities added or removed from the world), or
/// when there is only a small number of entities (~1 000 or so).
#[derive(Debug, Default)]
pub struct Naive {
    entities: Vec<(Entity, Vec3)>,
}

impl SpatialLookupAlgorithm for Naive {
    fn prepare(&mut self, entities: &[(Entity, Vec3)]) {
        self.entities = entities.to_owned();
    }

    fn entities_in_radius(&self, sample_point: Vec3, radius: f32) -> Vec<Entity> {
        let mut found_entities = Vec::new();

        for (entity, position) in &self.entities {
            if position.distance(sample_point) <= radius {
                found_entities.push(*entity);
            }
        }

        found_entities
    }
}
