//! Benchmarking for the BVH algorithm
use bevy::prelude::*;
use bevy_mod_spatial_query::prelude::*;
use bevy_mod_spatial_query::{algorithms, prepare_spatial_lookup};
use criterion::{
    AxisScale, BatchSize, BenchmarkId, Criterion, PlotConfiguration, SamplingMode, Throughput,
    black_box, criterion_group, criterion_main,
};
use turborand::prelude::*;

#[derive(Component, Debug)]
struct Marker;

const N_ELEMENTS_TO_TEST: &[usize] = &[100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000];

const WORLD_SIZE: f32 = 10.0;
const LOOKUP_RADIUS: f32 = 1.0;

/// Initialize a new World with `n` number of entities.
///
/// Entities are spawned in random positions, for large `n` this will result in uniform spread.
fn world_with_n_entities(n: usize) -> World {
    let rng = Rng::with_seed(417311532);
    let mut world = World::new();

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

    world
}

fn world_with_bvh(n: usize) -> (World, Schedule, Schedule) {
    let mut world = world_with_n_entities(n);
    let mut prepare_schedule = Schedule::default();
    let mut query_schedule = Schedule::default();

    world.insert_resource(SpatialLookupState::with_algorithm(
        algorithms::Bvh::default(),
    ));

    prepare_schedule.add_systems(prepare_spatial_lookup);
    query_schedule.add_systems(system_with_spatial_query);

    (world, prepare_schedule, query_schedule)
}

fn world_with_naive(n: usize) -> (World, Schedule, Schedule) {
    let mut world = world_with_n_entities(n);
    let mut prepare_schedule = Schedule::default();
    let mut query_schedule = Schedule::default();

    world.insert_resource(SpatialLookupState::with_algorithm(
        algorithms::Naive::default(),
    ));

    prepare_schedule.add_systems(prepare_spatial_lookup);
    query_schedule.add_systems(system_with_spatial_query);

    (world, prepare_schedule, query_schedule)
}

fn system_with_spatial_query(mut entities: SpatialQuery<Entity, With<Marker>>) {
    for entity in entities.in_radius(Vec3::ZERO, LOOKUP_RADIUS) {
        black_box(entity);
    }
}

fn benchmark_prepare_with_bvh(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    let mut group = c.benchmark_group("benchmark_prepare_with_bvh");
    group.sample_size(10);
    group.plot_config(plot_config);

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

fn compare_bvh_to_naive(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("compare_bvh_to_naive");
    group.sample_size(25);
    group.plot_config(plot_config);
    group.sampling_mode(SamplingMode::Flat);

    for n in N_ELEMENTS_TO_TEST {
        group.throughput(Throughput::Elements(*n as u64));

        group.bench_function(BenchmarkId::new("BVH", *n), |b| {
            b.iter_batched_ref(
                || world_with_bvh(*n),
                |(world, prepare_schedule, query_schedule)| {
                    prepare_schedule.run(world);

                    for _ in 0..100 {
                        query_schedule.run(world);
                    }
                },
                BatchSize::LargeInput,
            );
        });

        group.bench_function(BenchmarkId::new("Naive", *n), |b| {
            b.iter_batched_ref(
                || world_with_naive(*n),
                |(world, prepare_schedule, query_schedule)| {
                    prepare_schedule.run(world);

                    for _ in 0..100 {
                        query_schedule.run(world);
                    }
                },
                BatchSize::LargeInput,
            );
        });
    }
}

criterion_group!(benches, benchmark_prepare_with_bvh, compare_bvh_to_naive);
criterion_main!(benches);
