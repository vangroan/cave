#[macro_use]
extern crate criterion;

use criterion::{criterion_group, criterion_main, Criterion};

use cave::grid::{Grid, GridPosition};
use cave::pathfinder::{Cost, Pathfinder};

fn minus_one_benchmark(c: &mut Criterion) {
    c.bench_function("Bench Pathfinding 16x16x16 grid", |b| {
        b.iter(|| {
            let grid = Grid::with_size(16, 16, 16);
            let pathfinder = Pathfinder::new();
            pathfinder
                .find_path(
                    &grid,
                    &GridPosition::new(0, 0, 0),
                    &GridPosition::new(10, 10, 0),
                    |_, _| Cost::Passable,
                )
                .expect("Failed to find path");
        })
    });

    c.bench_function("Bench Pathfinding 128x128x128 grid", |b| {
        b.iter(|| {
            let grid = Grid::with_size(128, 128, 128);
            let pathfinder = Pathfinder::new();
            pathfinder
                .find_path(
                    &grid,
                    &GridPosition::new(0, 0, 0),
                    &GridPosition::new(10, 10, 0),
                    |_, _| Cost::Passable,
                )
                .expect("Failed to find path");
        })
    });
}

criterion_group!(benches, minus_one_benchmark);

criterion_main!(benches);
