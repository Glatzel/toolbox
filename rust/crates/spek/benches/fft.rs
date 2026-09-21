use std::ops::Range;

use criterion::BenchmarkId;

fn bench_fft(c: &mut criterion::Criterion) {
    for size in [10, 20, 30] {
        let data: Vec<f64> = Range::<f64>::new(0.0, 1.0).take(size).collect();
        group.bench_with_input(BenchmarkId::new("phastft_fft_2^{}", size), size, |b, _| {
            let backend = spek::fft_backend::phastft::PhastftBackend::new();
            b.iter(|| {
                backend.fft(&mut data.clone());
            })
        });
        group.bench_with_input(BenchmarkId::new("realfft_fft_2^{}", size), size, |b, _| {
            let backend = spek::fft_backend::realfft::RealfftBackend::new();
            b.iter(|| {
                backend.fft(&mut data.clone());
            })
        });
        group.bench_with_input(BenchmarkId::new("rustfft_fft_2^{}", size), size, |b, _| {
            let plannar = rustfft::FftPlanner::new();
            let backend = spek::fft_backend::rustfft::RustfftBackend::new(
                plannar.plan_fft_forward(data.len()),
                plannar.plan_fft_inverse(data.len()),
            );
            b.iter(|| {
                backend.fft(&mut data.clone());
            })
        });
    }
}
fn bench_ifft(c: &mut criterion::Criterion) {
    for size in [10, 20, 30] {
        let data: Vec<f64> = Range::<f64>::new(0.0, 1.0).take(size).collect();
        group.bench_with_input(BenchmarkId::new("phastft_ifft_2^{}", size), size, |b, _| {
            let backend = spek::fft_backend::phastft::PhastftBackend::new();
            let mut spectrum = backend.fft(&mut data.clone());
            b.iter(|| {
                backend.ifft(&mut spectrum);
            })
        });
        group.bench_with_input(BenchmarkId::new("realfft_ifft_2^{}", size), size, |b, _| {
            let backend = spek::fft_backend::realfft::RealfftBackend::new();
            let mut spectrum = backend.fft(&mut data.clone());
            b.iter(|| {
                backend.ifft(&mut spectrum);
            })
        });
        group.bench_with_input(BenchmarkId::new("rustfft_ifft_2^{}", size), size, |b, _| {
            let plannar = rustfft::FftPlanner::new();
            let backend = spek::fft_backend::rustfft::RustfftBackend::new(
                plannar.plan_fft_forward(data.len()),
                plannar.plan_fft_inverse(data.len()),
            );
            let mut spectrum = backend.fft(&mut data.clone());
            b.iter(|| {
                backend.ifft(&mut spectrum);
            })
        });
    }
}
criterion_group!(benches, bench_fft, bench_ifft);
criterion_main!(benches);
