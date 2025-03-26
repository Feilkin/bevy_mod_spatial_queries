//! Bounding Volume Hierarchy -accelerated spatial lookup

use crate::SpatialLookupAlgorithm;
use bevy::math::FloatPow;
use bevy::prelude::*;

type EntityPositionPair = (Entity, Vec3);

/// Bounding Volume Hierarchy -based spatial acceleration algorithm.
///
/// This implementation uses Surface Area Heuristic for splitting the space. Maximum number of
/// splits to sample can be configured with the `max_split_samples_per_axis` field. A reasonable
/// default of 10 is provided.
///
/// Number of entities per leaf node is controlled by the `entities_per_leaf` field. Storing higher
/// number of entities per field results in smaller tree structure, faster tree building and
/// traversal, but slower final entity filtering.
///
/// Spatial lookups with the BVH structure can be split into two phases: tree traversal and final
/// filtering.
///
/// During the traversal, the BVH tree is traversed starting from the node, and each node
/// that intersects with the query (radius, aabb, etc) is entered.
///
/// For each entered node, if it is a leaf node, each contained entity is then filtered against the
/// query (radius, aabb, etc) to remove entities which are contained in the leaf node but do not
/// actually intersect the query.
pub struct Bvh {
    /// Maximum number of entities per leaf node.
    pub entities_per_leaf: usize,
    /// Maximum number of test splits performed per axis. Larger number results in better (=faster)
    /// tree structure but makes tree generation slower.
    pub max_split_samples_per_axis: usize,
    root: Option<BvhNode>,
}

impl Default for Bvh {
    fn default() -> Self {
        Bvh {
            entities_per_leaf: 1000,
            max_split_samples_per_axis: 10,
            root: None,
        }
    }
}

impl SpatialLookupAlgorithm for Bvh {
    fn prepare(&mut self, entities: &[EntityPositionPair]) {
        let root = split_node(
            entities,
            self.entities_per_leaf,
            self.max_split_samples_per_axis,
        );

        self.root = Some(root);
    }

    fn entities_in_radius(&self, sample_point: Vec3, radius: f32) -> Vec<Entity> {
        if let Some(root) = &self.root {
            root.entities_in_radius(sample_point, radius)
        } else {
            warn!(
                "called Bvh::entities_in_radius before initializing the lookup with Bvh::prepare,\
                no entities will be returned"
            );
            Vec::new()
        }
    }
}

/// Recursively splits a slice of Entity, Position pairs into BVH nodes.
///
/// This implementation uses the Surface Area Heuristic with a user-controllable amount of
/// split samples.
fn split_node(
    entities: &[EntityPositionPair],
    entities_per_leaf: usize,
    max_split_samples_per_axis: usize,
) -> BvhNode {
    assert!(!entities.is_empty());

    // we make a copy of the slice, because we need to sort it to find the axis of best split
    let mut entities = entities.to_vec();
    let aabb = calculate_aabb(&entities);

    if entities.len() <= entities_per_leaf {
        return BvhNode {
            aabb,
            kind: BvhNodeKind::Leaf(entities),
        };
    }

    // find the axis of best split
    let x_index_and_cost = {
        entities.sort_by(|a, b| a.1.x.total_cmp(&b.1.x));
        find_split_index_and_cost(&entities, max_split_samples_per_axis)
    };
    let y_index_and_cost = {
        entities.sort_by(|a, b| a.1.y.total_cmp(&b.1.y));
        find_split_index_and_cost(&entities, max_split_samples_per_axis)
    };
    let z_index_and_cost = {
        entities.sort_by(|a, b| a.1.z.total_cmp(&b.1.z));
        find_split_index_and_cost(&entities, max_split_samples_per_axis)
    };

    // split entities at the index of best split
    let (left, right) =
        if x_index_and_cost.1 < y_index_and_cost.1 && x_index_and_cost.1 < z_index_and_cost.1 {
            entities.sort_by(|a, b| a.1.x.total_cmp(&b.1.x));
            entities.split_at(x_index_and_cost.0)
        } else if y_index_and_cost.1 < z_index_and_cost.1 {
            entities.sort_by(|a, b| a.1.y.total_cmp(&b.1.y));
            entities.split_at(y_index_and_cost.0)
        } else {
            entities.split_at(z_index_and_cost.0)
        };

    let left_node = split_node(left, entities_per_leaf, max_split_samples_per_axis);
    let right_node = split_node(right, entities_per_leaf, max_split_samples_per_axis);

    BvhNode {
        aabb,
        kind: BvhNodeKind::Branch(Box::new(left_node), Box::new(right_node)),
    }
}

/// Find the best split index and the resulting cost of the sorted `entities` slice.
fn find_split_index_and_cost(
    entities: &[EntityPositionPair],
    max_split_samples_per_axis: usize,
) -> (usize, f32) {
    assert!(entities.len() > 1);

    let samples = entities.len().min(max_split_samples_per_axis);
    let step = entities.len() / samples;

    let mut min = (1, f32::INFINITY);
    for i in (1..entities.len()).step_by(step) {
        let current_cost = cost(entities, i);
        if current_cost < min.1 {
            min = (i, current_cost);
        }
    }

    min
}

/// Surface Area Heuristic.
///
/// The cost is based on the surface areas of the two resulting AABB shapes.
fn cost(entities: &[EntityPositionPair], index: usize) -> f32 {
    let (left, right) = entities.split_at(index);

    calculate_aabb(left).total_surface_area() * (index as f32)
        + calculate_aabb(right).total_surface_area() * (entities.len() - index) as f32
}

/// Calculates the Axis-Aligned Bounding Box for a set of points.
fn calculate_aabb(entities: &[EntityPositionPair]) -> Aabb {
    let mut aabb = Aabb::ZERO;

    for (_, position) in entities {
        aabb.min = aabb.min.min(*position);
        aabb.max = aabb.max.max(*position);
    }

    aabb
}

/// Axis-Aligned Bounding Box.
struct Aabb {
    /// Left-bottom corner of the AABB
    min: Vec3,
    /// Top-right corner of the AABB
    max: Vec3,
}

impl Aabb {
    const ZERO: Aabb = Aabb {
        min: Vec3::ZERO,
        max: Vec3::ZERO,
    };

    pub fn total_surface_area(&self) -> f32 {
        let extents = self.max - self.min;

        extents.x * extents.y * 2. + extents.x * extents.z * 2. + extents.y * extents.z * 2.
    }
}

enum BvhNodeKind {
    Leaf(Vec<EntityPositionPair>),
    Branch(Box<BvhNode>, Box<BvhNode>),
}

/// Node of the BVH tree.
///
/// Each node contains an AABB (the chosen bounding volume),
/// and either a list of entities or 2 child nodes.
struct BvhNode {
    aabb: Aabb,
    kind: BvhNodeKind,
}

impl BvhNode {
    /// Returns a list of entities that are in radius of the given sample point.
    fn entities_in_radius(&self, sample_point: Vec3, radius: f32) -> Vec<Entity> {
        if !self.intersects_sphere(sample_point, radius) {
            return Vec::new();
        }

        match &self.kind {
            BvhNodeKind::Leaf(entity_position_pairs) => entity_position_pairs
                .iter()
                .filter_map(|(entity, position)| {
                    if position.distance(sample_point) <= radius {
                        Some(*entity)
                    } else {
                        None
                    }
                })
                .collect(),
            BvhNodeKind::Branch(left, right) => {
                let mut total = left.entities_in_radius(sample_point, radius);

                total.extend(right.entities_in_radius(sample_point, radius));

                total
            }
        }
    }

    /// Returns true if this node intersects given sphere.
    fn intersects_sphere(&self, sample_point: Vec3, radius: f32) -> bool {
        // implementation is based on Jim Arvo's algorithm from "Graphics Gems".
        // http://web.archive.org/web/20100323053111/http://www.ics.uci.edu/~arvo/code/BoxSphereIntersect.c
        let mut dmin = 0.;

        for axis in 0..3 {
            if sample_point[axis] < self.aabb.min[axis] {
                dmin += (sample_point[axis] - self.aabb.min[axis]).squared();
            } else if sample_point[axis] > self.aabb.max[axis] {
                dmin += (sample_point[axis] - self.aabb.max[axis]).squared();
            }
        }

        dmin <= radius.squared()
    }
}
