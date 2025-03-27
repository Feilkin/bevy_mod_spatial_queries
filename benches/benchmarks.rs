//! Benchmarking for the BVH algorithm
use bevy::prelude::*;
use bevy_mod_spatial_query::prelude::*;
use bevy_mod_spatial_query::{algorithms, prepare_spatial_lookup};
use criterion::{BatchSize, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use turborand::prelude::*;

#[derive(Component, Debug)]
struct Marker;

const N_ELEMENTS_TO_TEST: &[usize] = &[100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000];

const WORLD_SIZE: f32 = 10.0;
const LOOKUP_RADIUS: f32 = 1.0;

fn world_with_bvh(n: usize) -> (World, Schedule, Schedule) {
    let rng = Rng::with_seed(417311532);
    let mut world = World::new();
    let mut prepare_schedule = Schedule::default();
    let mut query_schedule = Schedule::default();

    world.insert_resource(SpatialLookupState::with_algorithm(
        algorithms::Bvh::default(),
    ));

    prepare_schedule.add_systems(prepare_spatial_lookup);
    query_schedule.add_systems(system_with_spatial_query);

    for _ in 0..n {
        world.spawn((
            Marker,
            GlobalTransform::from_xyz(
                rng.f32_normalized() * WORLD_SIZE,
                rng.f32_normalized() * WORLD_SIZE,
                rng.f32_normalized() * WORLD_SIZE,
            ),
        ));
    }

    (world, prepare_schedule, query_schedule)
}

fn system_with_spatial_query(mut entities: SpatialQuery<Entity, With<Marker>>) {
    for entity in entities.in_radius(Vec3::ZERO, LOOKUP_RADIUS) {
        std::hint::black_box(entity);
    }
}

fn benchmark_prepare_with_bvh(c: &mut Criterion) {
    let mut group = c.benchmark_group("benchmark_prepare_with_bvh");
    group.sample_size(10);

    for n in N_ELEMENTS_TO_TEST {
        group.throughput(Throughput::Elements(*n as u64));
        group.bench_function(BenchmarkId::from_parameter(*n), |b| {
            b.iter_batched_ref(
                || world_with_bvh(*n),
                |(world, prepare_schedule, _)| prepare_schedule.run(world),
                BatchSize::PerIteration,
            );
        });
    }
}

criterion_group!(benches, benchmark_prepare_with_bvh);
criterion_main!(benches);
