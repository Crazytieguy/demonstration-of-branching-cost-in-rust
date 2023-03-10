use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::random;
use shape::{get_area_branching, get_area_non_branching, Shape, ShapeType};

use ShapeType::*;

fn random_shape() -> Shape {
    Shape {
        shape_type: match (random(), random()) {
            (false, false) => Square,
            (true, false) => Rectangle,
            (false, true) => Triangle,
            (true, true) => Circle,
        },
        width: random(),
        height: random(),
    }
}

fn bench_area(c: &mut Criterion) {
    let mut group = c.benchmark_group("ShapeArea");
    let random_shapes: Vec<_> = (0..100000).map(|_| random_shape()).collect();
    let repetitions_str = format!("{}-repetitions", random_shapes.len());
    group.bench_with_input(
        BenchmarkId::new("Branching", &repetitions_str),
        &random_shapes,
        |b, shapes| {
            b.iter(|| {
                shapes.iter().for_each(|shape| {
                    black_box(get_area_branching(shape));
                })
            });
        },
    );
    group.bench_with_input(
        BenchmarkId::new("NonBranching", &repetitions_str),
        &random_shapes,
        |b, shapes| {
            b.iter(|| {
                shapes.iter().for_each(|shape| {
                    black_box(get_area_non_branching(shape));
                })
            })
        },
    );
    group.finish();
}

criterion_group!(benches, bench_area);
criterion_main!(benches);
