use criterion::{Criterion, criterion_group, criterion_main};
use nine_slice::{BorderOffsets, BorderScaling, NineSlicedSprite};
use std::io::Cursor;

fn criterion_benchmark(c: &mut Criterion) {
    let slices = BorderOffsets {
        left: 32,
        top: 32,
        right: 32,
        bottom: 32,
    };
    let mut sprite = NineSlicedSprite::from_png(
        Cursor::new(include_bytes!("../test_files/test_image.png")),
        slices,
        BorderScaling::Stretch,
    )
    .unwrap();
    c.bench_function("stretched edges", |b| {
        b.iter(|| {
            let _ = sprite.resize(1024, 768).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
