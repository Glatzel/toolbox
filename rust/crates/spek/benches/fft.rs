use criterion::{BenchmarkId, criterion_group, criterion_main};
use spek::fft_backend::IFftBackend;

fn bench_fft(c: &mut criterion::Criterion) {
    for size in [10usize, 15, 20, 25, 30] {
        let mut group = c.benchmark_group("fft");
        let data: Vec<f64> = (0..2.pow(size)).map(|i| i as f64).collect();
        group.bench_with_input(BenchmarkId::new("phastft_fft_2^", size), &size, |b, _| {
            let backend = spek::fft_backend::phastft::PhastftBackend::<
                phastft::planner::PlannerR2c32,
            >::new(data.len());
            b.iter(|| {
                backend.fft_unchecked(&mut data.clone());
            })
        });
        group.bench_with_input(BenchmarkId::new("realfft_fft_2^", size), &size, |b, _| {
            let backend = spek::fft_backend::realfft::RealfftBackend::new(bin_size);
            b.iter(|| {
                backend.fft_unchecked(&mut data.clone());
            })
        });
        group.bench_with_input(BenchmarkId::new("rustfft_fft_2^", size), &size, |b, _| {
            let plannar = rustfft::FftPlanner::new();
            let backend = spek::fft_backend::rustfft::RustfftBackend::new(
                plannar.plan_fft_forward(data.len()),
                plannar.plan_fft_inverse(data.len()),
            );
            b.iter(|| {
                backend.fft_unchecked(&mut data.clone());
            })
        });
    }
}
fn bench_ifft(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("ifft");
    for size in [10usize, 15, 20, 25, 30] {
        let data: Vec<f64> = (0..size).map(|i| i).collect();
        group.bench_with_input(BenchmarkId::new("phastft_ifft_2^", size), &size, |b, _| {
            let backend = spek::fft_backend::phastft::PhastftBackend::new();
            let mut spectrum = backend.fft_unchecked(&mut data.clone());
            b.iter(|| {
                backend.ifft_unchecked(&mut spectrum);
            })
        });
        group.bench_with_input(BenchmarkId::new("realfft_ifft_2^", size), &size, |b, _| {
            let backend = spek::fft_backend::realfft::RealfftBackend::new();
            let mut spectrum = backend.fft_unchecked(&mut data.clone());
            b.iter(|| {
                backend.ifft_unchecked(&mut spectrum);
            })
        });
        group.bench_with_input(BenchmarkId::new("rustfft_ifft_2^", size), &size, |b, _| {
            let plannar = rustfft::FftPlanner::new();
            let backend = spek::fft_backend::rustfft::RustfftBackend::new(
                plannar.plan_fft_forward(data.len()),
                plannar.plan_fft_inverse(data.len()),
            );
            let mut spectrum = backend.fft_unchecked(&mut data.clone());
            b.iter(|| {
                backend.ifft_unchecked(&mut spectrum);
            })
        });
    }
}
criterion_group!(benches, bench_fft, bench_ifft);
criterion_main!(benches);
