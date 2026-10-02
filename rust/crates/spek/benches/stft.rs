use std::fmt::Debug;
use std::marker::{Send, Sync};

use criterion::{BatchSize, BenchmarkId, criterion_group, criterion_main};
use generic_num::num;
use num_traits::{Float, FloatConst};
use spek::fft_backend::IFftBackend;
use spek::stft::Stft;
use spek::windows::Window::Hann;

const SIZE: [usize; 1] = [7];
const FFT_SIZE: [usize; 5] = [1024, 2048, 4096, 8192, 16384];

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
    let spectrum = stft.stft(&mut data);
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
                stft.par_stft(&mut data);
            })
        },
    );
    g.bench_with_input(
        BenchmarkId::new(format!("{}_istft_{}_10^", name, fft_size), size),
        &size,
        |b, _| {
            b.iter_batched(
                || spectrum.clone(),
                |mut spectrum| {
                    stft.istft(&mut spectrum);
                },
                BatchSize::SmallInput,
            );
        },
    );
    g.bench_with_input(
        BenchmarkId::new(format!("{}_istft_parallel_{}_10^", name, fft_size), size),
        &size,
        |b, _| {
            b.iter_batched(
                || spectrum.clone(),
                |mut spectrum| {
                    stft.par_istft(&mut spectrum);
                },
                BatchSize::SmallInput,
            );
        },
    );
}
fn bench_f32(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("f32");
    for size in SIZE {
        for fft_size in FFT_SIZE {
            #[cfg(feature = "phastft")]
            bench_wrapper::<f32, _>(
                &mut group,
                "phastft",
                spek::fft_backend::phastft::PhastftBackend::<phastft::planner::PlannerR2c32>::new(
                    fft_size,
                ),
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
    }
}
fn bench_f64(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("f64");
    for size in SIZE {
        for fft_size in FFT_SIZE {
            #[cfg(feature = "phastft")]
            bench_wrapper::<f64, _>(
                &mut group,
                "phastft",
                spek::fft_backend::phastft::PhastftBackend::<phastft::planner::PlannerR2c64>::new(
                    fft_size,
                ),
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
    }
}
criterion_group!(benches, bench_f32, bench_f64);
criterion_main!(benches);
