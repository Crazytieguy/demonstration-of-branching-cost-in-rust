use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::random;
use shape::{get_area_switch, get_area_union, Shape, ShapeType};

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
    let mut group = c.benchmark_group("Shape Area");
    let random_shapes: Vec<_> = (0..100000).map(|_| random_shape()).collect();
    group.bench_with_input(
        BenchmarkId::new("Switch", random_shapes.len()),
        &random_shapes,
        |b, shapes| {
            b.iter(|| {
                shapes.iter().for_each(|shape| {
                    black_box(get_area_switch(shape));
                })
            });
        },
    );
    group.bench_with_input(
        BenchmarkId::new("Union", random_shapes.len()),
        &random_shapes,
        |b, shapes| {
            b.iter(|| {
                shapes.iter().for_each(|shape| {
                    black_box(get_area_union(shape));
                })
            })
        },
    );
    group.finish();
}

criterion_group!(benches, bench_area);
criterion_main!(benches);
