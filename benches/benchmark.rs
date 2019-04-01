#[macro_use]
extern crate criterion;

use criterion::{criterion_group, criterion_main, Criterion};

use cave::grid::{Grid, GridPosition};
use cave::pathfinding::{AStar, Locomotion, NoOpCost, NoOpLocomotion, Pathfinder, GO_ANYWHERE};

fn pathfinding_benchmark(c: &mut Criterion) {
    c.bench_function("Bench Pathfinding 16x16x16 grid", |b| {
        b.iter(|| {
            let grid = Grid::with_size(16, 16, 16);
            let pathfinder = AStar::new();
            pathfinder.find_path(
                &grid,
                &Locomotion::new(&[GO_ANYWHERE]),
                &GridPosition::new(0, 0, 0),
                &GridPosition::new(10, 10, 0),
                &NoOpCost,
                &NoOpLocomotion,
            );
        })
    });

    c.bench_function("Bench Pathfinding 128x128x128 grid", |b| {
        b.iter(|| {
            let grid = Grid::with_size(128, 128, 128);
            let pathfinder = AStar::new();
            pathfinder.find_path(
                &grid,
                &Locomotion::new(&[GO_ANYWHERE]),
                &GridPosition::new(0, 0, 0),
                &GridPosition::new(10, 10, 0),
                &NoOpCost,
                &NoOpLocomotion,
            );
        })
    });
}

fn depth_sort_benchmark(c: &mut Criterion) {
    use cave::pigeon::*;

    c.bench_function("Bench Pigeonhole Sort 16x16x16 grid", |b| {
        b.iter(|| {
            let max = 16 * 16 * 16;
            // (depth, identifier)
            let mut pigeon = PigeonholeSort::<(i32, i32)>::new(0, max);
            let mut source: Vec<(i32, i32)> = (0..max * 4).map(|i| (i / 4, i)).collect();
            let mut target: Vec<(i32, i32)> = vec![Default::default(); source.len()];

            pigeon.sort_into(&mut source, &mut target, |pair| pair.0);
        });
    });

    c.bench_function("Bench Pigeonhole Sort 64x64x64 grid", |b| {
        b.iter(|| {
            let max = 64 * 64 * 64;
            // (depth, identifier)
            let mut pigeon = PigeonholeSort::<(i32, i32)>::new(0, max);
            let mut source: Vec<(i32, i32)> = (0..max * 4).map(|i| (i / 4, i)).collect();
            let mut target: Vec<(i32, i32)> = vec![Default::default(); source.len()];

            pigeon.sort_into(&mut source, &mut target, |pair| pair.0);
        });
    });
}

criterion_group!(benches, pathfinding_benchmark, depth_sort_benchmark);

criterion_main!(benches);
