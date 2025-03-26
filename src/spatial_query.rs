use crate::{SpatialLookupState, SpatialQueryIterator};
use bevy::ecs::query::{QueryData, QueryFilter};
use bevy::ecs::system::SystemParam;
use bevy::math::Vec3;
use bevy::prelude::{Query, Res};

#[derive(SystemParam)]
pub struct SpatialQuery<'w, 's, D: QueryData + 'static, F: QueryFilter + 'static = ()> {
    lookup: Res<'w, SpatialLookupState>,
    query: Query<'w, 's, D, F>,
}

impl<'w, 's, D: QueryData + 'static, F: QueryFilter + 'static> SpatialQuery<'w, 's, D, F> {
    pub fn in_radius<'q>(
        &'q mut self,
        sample_point: Vec3,
        radius: f32,
    ) -> SpatialQueryIterator<'w, 's, 'q, D, F> {
        let entities_in_range = self.lookup.entities_in_radius(sample_point, radius);

        SpatialQueryIterator::with_entities(entities_in_range, &mut self.query)
    }
}
