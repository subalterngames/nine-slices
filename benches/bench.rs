use blittle::Rgb8Surface;
use blittle::png::Png;
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

    let mut surface = NineSlicedSprite::new(
        Rgb8Surface::read_png(Cursor::new(include_bytes!("../test_files/src/stretch.png")))
            .unwrap(),
        slices,
        BorderScaling::Stretch,
    )
    .unwrap();
    c.bench_function("stretched borders", |b| {
        b.iter(|| {
            let _ = surface.resize(1024, 768).unwrap();
        })
    });

    let mut surface = NineSlicedSprite::new(
        Rgb8Surface::read_png(Cursor::new(include_bytes!("../test_files/src/stretch.png")))
            .unwrap(),
        slices,
        BorderScaling::Stretch,
    )
    .unwrap();
    surface.set_resize_algorithm(ResizeAlg::Nearest);
    c.bench_function("stretched borders nearest", |b| {
        b.iter(|| {
            let _ = surface.resize(1024, 768).unwrap();
        })
    });

    let mut surface = NineSlicedSprite::new(
        Rgb8Surface::read_png(Cursor::new(include_bytes!("../test_files/src/stretch.png")))
            .unwrap(),
        slices,
        BorderScaling::Stretch,
    )
    .unwrap();
    surface.set_resize_algorithm(ResizeAlg::Nearest);
    c.bench_function("stretched borders filled", |b| {
        b.iter(|| {
            let _ = surface.resize(1024, 768).unwrap();
        })
    });

    let mut surface = NineSlicedSprite::new(
        Rgb8Surface::read_png(Cursor::new(include_bytes!("../test_files/src/repeat.png"))).unwrap(),
        slices,
        BorderScaling::Repeat,
    )
    .unwrap();
    c.bench_function("repeated borders", |b| {
        b.iter(|| {
            let _ = surface.resize(1024, 768).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
