//! Spatially aware Queries for the Bevy game engine

use bevy::ecs::query::{QueryData, QueryFilter};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

mod spatial_query;

pub mod prelude {
    pub use crate::SpatialQueriesPlugin;
    pub use crate::spatial_query::SpatialQuery;
}

pub struct SpatialQueriesPlugin;

impl Plugin for SpatialQueriesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpatialLookupState::default())
            .add_systems(First, build_spatial_lookup);
    }
}

pub struct SpatialQueryIterator<'w, 's, 'q, D: QueryData + 'static, F: QueryFilter + 'static> {
    entities: Vec<Entity>,
    query: &'q mut Query<'w, 's, D, F>,
}

impl<'w, 's, 'q, D: QueryData + 'static, F: QueryFilter + 'static>
    SpatialQueryIterator<'w, 's, 'q, D, F>
{
    fn with_entities(entities: Vec<Entity>, query: &'q mut Query<'w, 's, D, F>) -> Self {
        SpatialQueryIterator { entities, query }
    }
}

impl<'w, 's, 'q, D: QueryData + 'static, F: QueryFilter + 'static> Iterator
    for SpatialQueryIterator<'w, 's, 'q, D, F>
where
    'w: 'q,
    's: 'q,
{
    type Item = D::Item<'q>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(entity) = self.entities.pop() {
            match unsafe { self.query.get_unchecked(entity) } {
                Ok(data) => return Some(unsafe { std::mem::transmute(data) }),
                Err(_) => continue,
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.entities.len()))
    }
}

#[derive(Resource, Default)]
pub struct SpatialLookupState {
    entities: Vec<(Entity, Vec3)>,
}

impl SpatialLookupState {
    pub fn entities_in_radius(&self, sample_point: Vec3, radius: f32) -> Vec<Entity> {
        let mut out = Vec::new();

        for (entity, position) in &self.entities {
            if position.distance(sample_point) < radius {
                out.push(*entity);
            }
        }

        out
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
}
