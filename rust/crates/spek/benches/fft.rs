use criterion::{BenchmarkId, criterion_group, criterion_main};
use spek::fft_backend::IFftBackend;

fn bench_fft(c: &mut criterion::Criterion) {
    for size in [10usize, 15, 20, 25, 30] {
        let mut group = c.benchmark_group("fft");
        let data: Vec<f64> = (0..2usize.pow(size as u32)).map(|i| i as f64).collect();
        group.bench_with_input(BenchmarkId::new("phastft_fft_2^", size), &size, |b, _| {
            let backend = spek::fft_backend::phastft::PhastftBackend::<
                phastft::planner::PlannerR2c64,
            >::new(data.len());
            let mut spectrum = backend.new_spectrum();
            let mut scratch = backend.new_forward_scratch();
            b.iter(|| {
                backend.fft_unchecked(&mut data.clone(), &mut spectrum, &mut scratch);
            })
        });
        group.bench_with_input(BenchmarkId::new("realfft_fft_2^", size), &size, |b, _| {
            let backend = spek::fft_backend::realfft::RealfftBackend::new(data.len());
            let mut spectrum = backend.new_spectrum();
            let mut scratch = backend.new_forward_scratch();
            b.iter(|| {
                backend.fft_unchecked(&mut data.clone(), &mut spectrum, &mut scratch);
            })
        });
        group.bench_with_input(BenchmarkId::new("rustfft_fft_2^", size), &size, |b, _| {
            let mut plannar = rustfft::FftPlanner::<f64>::new();
            let backend = spek::fft_backend::rustfft::RustfftBackend::new(
                plannar.plan_fft_forward(data.len()),
                plannar.plan_fft_inverse(data.len()),
            )
            .unwrap();
            let mut spectrum = backend.new_spectrum();
            let mut scratch = backend.new_forward_scratch();
            b.iter(|| {
                backend.fft_unchecked(&mut data.clone(), &mut spectrum, &mut scratch);
            })
        });
    }
}
fn bench_ifft(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("ifft");
    for size in [10usize, 15, 20, 25, 30] {
        let data: Vec<f64> = (0..2usize.pow(size as u32)).map(|i| i as f64).collect();
        group.bench_with_input(BenchmarkId::new("phastft_ifft_2^", size), &size, |b, _| {
            let backend = spek::fft_backend::phastft::PhastftBackend::<
                phastft::planner::PlannerR2c64,
            >::new(data.len());
            let mut spectrum = backend.new_spectrum();
            let mut scratch = backend.new_forward_scratch();
            backend.fft_unchecked(&mut data.clone(), &mut spectrum, &mut scratch);
            let mut signal = backend.new_signal();
            let mut scratch = backend.new_inverse_scratch();
            b.iter(|| {
                backend.ifft_unchecked(&mut spectrum, &mut signal, &mut scratch);
            })
        });
        group.bench_with_input(BenchmarkId::new("realfft_ifft_2^", size), &size, |b, _| {
            let backend = spek::fft_backend::realfft::RealfftBackend::new(data.len());
            let mut spectrum = backend.new_spectrum();
            let mut scratch = backend.new_forward_scratch();
            backend.fft_unchecked(&mut data.clone(), &mut spectrum, &mut scratch);
            let mut signal = backend.new_signal();
            let mut scratch = backend.new_inverse_scratch();
            b.iter(|| {
                backend.ifft_unchecked(&mut spectrum, &mut signal, &mut scratch);
            })
        });
        group.bench_with_input(BenchmarkId::new("rustfft_ifft_2^", size), &size, |b, _| {
            let mut plannar = rustfft::FftPlanner::<f64>::new();
            let backend = spek::fft_backend::rustfft::RustfftBackend::new(
                plannar.plan_fft_forward(data.len()),
                plannar.plan_fft_inverse(data.len()),
            )
            .unwrap();
            let mut spectrum = backend.new_spectrum();
            let mut scratch = backend.new_forward_scratch();
            backend.fft_unchecked(&mut data.clone(), &mut spectrum, &mut scratch);
            let mut signal = backend.new_signal();
            let mut scratch = backend.new_inverse_scratch();
            b.iter(|| {
                backend.ifft_unchecked(&mut spectrum, &mut signal, &mut scratch);
            })
        });
    }
}
criterion_group!(benches, bench_fft, bench_ifft);
criterion_main!(benches);
