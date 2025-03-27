//! Spatial lookup algorithms
//!
//! You can implement your own algorithm by implementing the `SpatialLookupAlgorithm` trait.

mod bvh;
mod naive;

// Re-export algorithms for ease of use.
pub use bvh::Bvh;
pub use naive::Naive;
