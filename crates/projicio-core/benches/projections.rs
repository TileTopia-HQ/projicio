use criterion::{Criterion, criterion_group, criterion_main};
use projicio_core::*;
use std::hint::black_box;

fn bench_web_mercator_forward(c: &mut Criterion) {
    let wm = WebMercator::new();
    let geo = Geographic::new(-0.1_f64.to_radians(), 51.5_f64.to_radians());

    c.bench_function("web_mercator_forward", |b| {
        b.iter(|| wm.forward(black_box(geo)))
    });
}

fn bench_utm_forward(c: &mut Criterion) {
    let tm = TransverseMercator::utm(30, true);
    let geo = Geographic::new(-0.1_f64.to_radians(), 51.5_f64.to_radians());

    c.bench_function("utm_zone30_forward", |b| {
        b.iter(|| tm.forward(black_box(geo)))
    });
}

fn bench_geodetic_to_geocentric(c: &mut Criterion) {
    c.bench_function("geodetic_to_geocentric", |b| {
        b.iter(|| {
            geodetic_to_geocentric(
                black_box(51.5_f64.to_radians()),
                black_box(-0.1_f64.to_radians()),
                black_box(0.0),
                black_box(&Ellipsoid::WGS84),
            )
        })
    });
}

criterion_group!(
    benches,
    bench_web_mercator_forward,
    bench_utm_forward,
    bench_geodetic_to_geocentric
);
criterion_main!(benches);
