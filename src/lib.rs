//! Spatially aware Queries for the Bevy game engine
//!
//! ```
//! # use bevy::prelude::*;
//! # use bevy_mod_spatial_query::prelude::*;
//! #
//! # #[derive(Component)]
//! # struct Mouse { position: Vec3 }
//! # #[derive(Component)]
//! # struct Circle;
//! #
//! fn change_color_on_hover(
//!      mouse: Single<&Mouse>,
//!      mut circles: SpatialQuery<&mut Circle>,
//!  ) {
//!     for mut circle in circles.in_radius(mouse.position, 10.) {
//!         // Do something with the circle..
//!     }
//! }
//! ```
//!
//! This crate aims to provide an ergonomic and fast way of performing spatial queries, i.e.
//! "nearby entities" -type queries. "Spatial" here refers purely to the `GlobalPosition` of an
//! entity, and does not consider things like meshes or collision shapes.
//!
//! By default, this crate uses a naive lookup algorithm as it outperforms more advanced algorithms
//! for simple (less than 1 000 000 entities) scenes with few (less than 100) queries. A BVH-based
//! algorithm is also provided, but it is only useful for scenes with many (100 000 000+) entities
//! or very many queries (10 000+). Users can implement their own lookup algorithms by implementing
//! the `SpatialLookupAlgorithm` trait, and inserting the `SpatialLookupState` resource like so:
//! ```
//! # use bevy::prelude::*;
//! # use bevy_mod_spatial_query::prelude::*;
//! #
//! # struct YourAwesomeAlgorithm;
//! #
//! # impl SpatialLookupAlgorithm for YourAwesomeAlgorithm {
//! #     fn prepare(&mut self, entities: &[(Entity, Vec3)]) {
//! #        todo!()
//! #     }
//! #
//! #     fn entities_in_radius(&self, sample_point: Vec3, radius: f32) -> Vec<Entity> {
//! #         todo!()
//! #     }
//! # }
//! #
//! # let mut app = App::new();
//! #
//! app.insert_resource(SpatialLookupState::with_algorithm(YourAwesomeAlgorithm));
//! ```
//!
use bevy::prelude::*;

pub mod algorithms;
mod spatial_query;
mod spatial_query_iterator;

pub mod prelude {
    pub use crate::spatial_query::SpatialQuery;
    pub use crate::spatial_query_iterator::SpatialQueryIterator;
    pub use crate::{SpatialLookupAlgorithm, SpatialLookupState, SpatialQueriesPlugin};
}

/// Adds `SpatialQuery` support to bevy.
pub struct SpatialQueriesPlugin;

/// System set for systems used to set up the spatial lookup.
///
/// All systems using `SpatialQuery<_>` *MUST* be scheduled after this set, i.e.
/// `.add_systems(First, your_awesome_system.after(PrepareSpatialLookup))`.
///
/// Manually specifying the `.after()` is only necessary for systems in the `First` schedule.
#[derive(SystemSet, Clone, Debug, Hash, PartialEq, Eq)]
pub struct PrepareSpatialLookup;

impl Plugin for SpatialQueriesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpatialLookupState::default())
            .add_systems(First, prepare_spatial_lookup.in_set(PrepareSpatialLookup));
    }
}

/// Trait for defining Spatial Lookup Algorithms to be used with `SpatialQuery<_>`.
pub trait SpatialLookupAlgorithm {
    /// Prepares the lookup algorithm with a fresh set of entities and their positions.
    ///
    /// This gets called once per frame in the `First` schedule, and therefore
    /// the implementation should be fairly fast. The algorithm should implement its own change
    /// detection if necessary.
    fn prepare(&mut self, entities: &[(Entity, Vec3)]);

    /// Returns a list of all entities that are within the given radius of the sample point.
    ///
    /// This method *MUST* return all entities within the radius of the sample point, and it *MUST*
    /// not return any entities outside of it.
    fn entities_in_radius(&self, sample_point: Vec3, radius: f32) -> Vec<Entity>;

    /// Draw debug gizmos
    fn debug_gizmos(&self, _gizmos: &mut Gizmos) {}
}

/// Resource which holds the configured `SpatialLookupAlgorithm` and relevant state.
#[derive(Resource)]
pub struct SpatialLookupState {
    pub entities: Vec<(Entity, Vec3)>,
    pub algorithm: Box<dyn SpatialLookupAlgorithm + Send + Sync>,
}

impl Default for SpatialLookupState {
    fn default() -> Self {
        SpatialLookupState {
            entities: Vec::new(),
            algorithm: Box::new(algorithms::Naive::default()),
        }
    }
}

impl SpatialLookupState {
    pub fn with_algorithm<T: SpatialLookupAlgorithm + Send + Sync + 'static>(algorithm: T) -> Self {
        Self {
            entities: vec![],
            algorithm: Box::new(algorithm),
        }
    }

    /// Returns a list of entities in the radius of the sample point.
    pub fn entities_in_radius(&self, sample_point: Vec3, radius: f32) -> Vec<Entity> {
        self.algorithm.entities_in_radius(sample_point, radius)
    }

    /// Prepares the configured algorithm for lookup.
    pub fn prepare_algorithm(&mut self) {
        self.algorithm.prepare(&self.entities);
    }
}

/// Prepares the configured spatial lookup algorithm.
///
/// Any systems using `SpatialQuery<_>` *MUST* be scheduled after this system
pub fn prepare_spatial_lookup(
    all_entities: Query<(Entity, &GlobalTransform)>,
    mut lookup_state: ResMut<SpatialLookupState>,
) {
    lookup_state.entities.clear();

    for (entity, transform) in &all_entities {
        lookup_state
            .entities
            .push((entity, transform.translation()));
    }

    lookup_state.prepare_algorithm();
}

pub fn draw_spatial_lookup_gizmos(lookup_state: Res<SpatialLookupState>, mut gizmos: Gizmos) {
    lookup_state.algorithm.debug_gizmos(&mut gizmos);
}
