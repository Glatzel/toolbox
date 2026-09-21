use std::fmt::Debug;
use std::marker::{Send, Sync};

use criterion::{BenchmarkId, criterion_group, criterion_main};
use generic_num::num;
use num_traits::{Float, FloatConst};
use spek::fft_backend::IFftBackend;
use spek::stft::Stft;
use spek::windows::Window::Hann;
fn get_size() -> usize {
    match (std::env::var("CI"), std::env::var("STFT_SIGNAL_SIZE")) {
        (Ok(_), _) => 6,
        (_, Ok(val)) => val.parse().unwrap_or(6),
        _ => 6,
    }
}
fn get_fft_size() -> usize {
    match (std::env::var("CI"), std::env::var("FFT_SIZE")) {
        (Ok(_), _) => 2048,
        (_, Ok(val)) => val.parse().unwrap_or(2048),
        _ => 2048,
    }
}
fn bench_wrapper<T, B>(
    g: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    name: &str,
    backend: B,
    size: usize,
) where
    T: Float + Debug + FloatConst + Sync + Send,
    B: IFftBackend<T> + Sync,
{
    let fft_size = backend.fft_size();
    let stft = Stft::new(fft_size, fft_size, Hann, backend).unwrap();
    let mut data = (0..10usize.pow(size as u32))
        .map(|i| num!(i))
        .collect::<Vec<_>>();
    g.bench_with_input(
        BenchmarkId::new(format!("{}_stft_{}_10^", name, fft_size), size),
        &size,
        |b, _| {
            b.iter(|| {
                stft.stft(&mut data);
            })
        },
    );
    g.bench_with_input(
        BenchmarkId::new(format!("{}_stft_parallel_{}_10^", name, fft_size), size),
        &size,
        |b, _| {
            b.iter(|| {
                stft.stft_parallel(&mut data);
            })
        },
    );
}
fn bench_f32(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("f32");
    let size = get_size();
    let fft_size = get_fft_size();
    #[cfg(feature = "phastft")]
    bench_wrapper::<f32, _>(
        &mut group,
        "phastft",
        spek::fft_backend::phastft::PhastftBackend::<phastft::planner::PlannerR2c32>::new(fft_size),
        size,
    );
    #[cfg(feature = "realfft")]
    bench_wrapper::<f32, _>(
        &mut group,
        "realfft",
        spek::fft_backend::realfft::RealfftBackend::new(fft_size),
        size,
    );
    #[cfg(feature = "rustfft")]
    let mut planner = rustfft::FftPlanner::new();
    #[cfg(feature = "rustfft")]
    bench_wrapper::<f32, _>(
        &mut group,
        "rustfft",
        spek::fft_backend::rustfft::RustfftBackend::new(
            planner.plan_fft_forward(fft_size),
            planner.plan_fft_inverse(fft_size),
        )
        .unwrap(),
        size,
    );
}
fn bench_f64(c: &mut criterion::Criterion) {
    let size = get_size();
    let fft_size = get_fft_size();
    let mut group = c.benchmark_group("f64");
    #[cfg(feature = "phastft")]
    bench_wrapper::<f64, _>(
        &mut group,
        "phastft",
        spek::fft_backend::phastft::PhastftBackend::<phastft::planner::PlannerR2c64>::new(fft_size),
        size,
    );
    #[cfg(feature = "realfft")]
    bench_wrapper::<f64, _>(
        &mut group,
        "realfft",
        spek::fft_backend::realfft::RealfftBackend::new(fft_size),
        size,
    );
    #[cfg(feature = "rustfft")]
    let mut planner = rustfft::FftPlanner::new();
    #[cfg(feature = "rustfft")]
    bench_wrapper::<f64, _>(
        &mut group,
        "rustfft",
        spek::fft_backend::rustfft::RustfftBackend::new(
            planner.plan_fft_forward(fft_size),
            planner.plan_fft_inverse(fft_size),
        )
        .unwrap(),
        size,
    );
}
criterion_group!(benches, bench_f32, bench_f64);
criterion_main!(benches);
