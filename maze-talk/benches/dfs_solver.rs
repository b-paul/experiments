use criterion::{Criterion, black_box, criterion_group, criterion_main};

use maze_talk::maze::Maze;
use maze_talk::solvers::min_dist_array_dfs;

use rand::prelude::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::from_seed([0x41; 32]);
    c.bench_function("dfs", |b| {
        b.iter_batched(
            || {
                let maze = Maze::random_with_rng(4, &mut rng);
                maze.grid()
            },
            |grid| black_box(min_dist_array_dfs(grid.clone(), 9)),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
