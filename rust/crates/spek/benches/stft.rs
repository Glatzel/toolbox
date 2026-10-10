use std::fmt::Debug;
use std::iter::Sum;
use std::marker::{Send, Sync};

use criterion::{BenchmarkId, criterion_group, criterion_main};
use generic_num::num;
use num_traits::{Float, FloatConst};
use spek::data_types::Signal;
use spek::fft::IFftBackend;
use spek::pad::Pad;
use spek::spectrum::Stft;
use spek::windows::Window::Hann;

const SIZE: [usize; 1] = [7];
const FFT_SIZE: [usize; 5] = [1024, 2048, 4096, 8192, 16384];

fn bench_wrapper<T, B>(
    g: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    name: &str,
    backend: B,
    size: usize,
) where
    T: Float + Debug + FloatConst + Sync + Sum + Send,
    B: IFftBackend<T> + Sync,
{
    let fft_size = backend.fft_size();
    let stft = Stft::new(fft_size, fft_size, Hann, Pad::default(), false, backend).unwrap();
    let data: Signal<T> = (0..10usize.pow(size as u32))
        .map(|i| num!(i))
        .collect::<Vec<_>>()
        .into();
    let mut spectrum = stft.stft(data.as_ref());
    g.bench_with_input(
        BenchmarkId::new(format!("{}_stft_{}_10^", name, fft_size), size),
        &size,
        |b, _| b.iter(|| stft.stft(data.as_ref())),
    );
    g.bench_with_input(
        BenchmarkId::new(format!("{}_stft_parallel_{}_10^", name, fft_size), size),
        &size,
        |b, _| b.iter(|| stft.par_stft(data.as_ref())),
    );
    g.bench_with_input(
        BenchmarkId::new(format!("{}_istft_{}_10^", name, fft_size), size),
        &size,
        |b, _| {
            b.iter(|| stft.istft(&mut spectrum));
        },
    );
    g.bench_with_input(
        BenchmarkId::new(format!("{}_istft_parallel_{}_10^", name, fft_size), size),
        &size,
        |b, _| {
            b.iter(|| stft.par_istft(&mut spectrum));
        },
    );
}
fn bench_f32(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("f32");
    for size in SIZE {
        for fft_size in FFT_SIZE {
            bench_wrapper::<f32, _>(
                &mut group,
                "phastft",
                spek::fft::PhastftBackend::<phastft::planner::PlannerR2c32>::new(fft_size),
                size,
            );
        }
    }
}
fn bench_f64(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("f64");
    for size in SIZE {
        for fft_size in FFT_SIZE {
            bench_wrapper::<f64, _>(
                &mut group,
                "phastft",
                spek::fft::PhastftBackend::<phastft::planner::PlannerR2c64>::new(fft_size),
                size,
            );
        }
    }
}

criterion_group!(benches, bench_f32, bench_f64);
criterion_main!(benches);
