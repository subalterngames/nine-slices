use criterion::{Criterion, criterion_group, criterion_main};
use fast_image_resize::ResizeAlg;
use nine_slices::{BorderOffsets, BorderScaling, NineSlicedSprite};
use std::io::Cursor;

fn criterion_benchmark(c: &mut Criterion) {
    let slices = BorderOffsets {
        left: 32,
        top: 32,
        right: 32,
        bottom: 32,
    };

    let mut sprite = NineSlicedSprite::from_png(
        Cursor::new(include_bytes!("../test_files/src/stretch.png")),
        slices,
        BorderScaling::Stretch,
    )
    .unwrap();
    c.bench_function("stretched borders", |b| {
        b.iter(|| {
            let _ = sprite.resize(1024, 768).unwrap();
        })
    });

    let mut sprite = NineSlicedSprite::from_png(
        Cursor::new(include_bytes!("../test_files/src/stretch.png")),
        slices,
        BorderScaling::Stretch,
    )
    .unwrap();
    sprite.set_resize_algorithm(ResizeAlg::Nearest);
    c.bench_function("stretched borders nearest", |b| {
        b.iter(|| {
            let _ = sprite.resize(1024, 768).unwrap();
        })
    });

    let mut sprite = NineSlicedSprite::from_png(
        Cursor::new(include_bytes!("../test_files/src/solid.png")),
        slices,
        BorderScaling::Stretch,
    )
    .unwrap();
    sprite.set_resize_algorithm(ResizeAlg::Nearest);
    c.bench_function("stretched borders filled", |b| {
        b.iter(|| {
            let _ = sprite.resize(1024, 768).unwrap();
        })
    });

    let mut sprite = NineSlicedSprite::from_png(
        Cursor::new(include_bytes!("../test_files/src/repeat.png")),
        slices,
        BorderScaling::Repeat,
    )
    .unwrap();
    c.bench_function("repeated borders", |b| {
        b.iter(|| {
            let _ = sprite.resize(1024, 768).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
