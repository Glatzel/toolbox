use criterion::{BenchmarkId, criterion_group, criterion_main};
use generic_num::num;
use num_traits::Float;
use spek::fft_backend::IFftBackend;
use spek::spectogram::{ISpectrogram, Spectrogram};
const SIZE: [usize; 4] = [5, 10, 15, 20];

fn bench_wrapper<T, B, SP>(
    g: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    name: &str,
    backend: B,
    size: usize,
) where
    T: Float,
    B: IFftBackend<T, SP>,
    Spectrogram<SP>: ISpectrogram<SP>,
    SP: Clone,
{
    let data: Vec<T> = (0..2usize.pow(size as u32)).map(|i| num!(i)).collect();
    g.bench_with_input(
        BenchmarkId::new(format!("{}_fft_2^", name), size),
        &size,
        |b, _| {
            let mut spectrum = backend.new_spectrum();
            let mut scratch = backend.new_forward_scratch();
            b.iter(|| {
                backend.fft_unchecked(&mut data.clone(), &mut spectrum, &mut scratch);
            })
        },
    );
    g.bench_with_input(
        BenchmarkId::new(format!("{}_ifft_2^", std::any::type_name::<B>()), size),
        &size,
        |b, _| {
            let mut spectrum = backend.new_spectrum();
            let mut scratch = backend.new_forward_scratch();
            backend.fft_unchecked(&mut data.clone(), &mut spectrum, &mut scratch);
            let mut signal = backend.new_signal();
            let mut scratch = backend.new_inverse_scratch();
            b.iter(|| {
                backend.ifft_unchecked(&mut spectrum.clone(), &mut signal, &mut scratch);
            })
        },
    );
}
fn bench_f32(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("f32");
    for size in SIZE {
        let bsize = 2usize.pow(size as u32);
        bench_wrapper::<f32, _, _>(
            &mut group,
            "phastft",
            spek::fft_backend::phastft::PhastftBackend::<phastft::planner::PlannerR2c32>::new(
                bsize,
            ),
            size,
        );
        bench_wrapper::<f32, _, _>(
            &mut group,
            "realfft"
            spek::fft_backend::realfft::RealfftBackend::new(bsize),
            size,
        );
        let mut planner = rustfft::FftPlanner::new();
        bench_wrapper::<f32, _, _>(
            &mut group,
            "rustfft",
            spek::fft_backend::rustfft::RustfftBackend::new(
                planner.plan_fft_forward(bsize),
                planner.plan_fft_inverse(bsize),
            )
            .unwrap(),
            size,
        );
    }
}
fn bench_f64(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("f64");
    for size in SIZE {
        let bsize = 2usize.pow(size as u32);
        bench_wrapper::<f64, _, _>(
            &mut group,
            "phastft",
            spek::fft_backend::phastft::PhastftBackend::<phastft::planner::PlannerR2c64>::new(
                bsize,
            ),
            size,
        );
        bench_wrapper::<f64, _, _>(
            &mut group,
            "realfft",
            spek::fft_backend::realfft::RealfftBackend::new(bsize),
            size,
        );
        let mut planner = rustfft::FftPlanner::new();
        bench_wrapper::<f64, _, _>(
            &mut group,
            "rustfft",
            spek::fft_backend::rustfft::RustfftBackend::new(
                planner.plan_fft_forward(bsize),
                planner.plan_fft_inverse(bsize),
            )
            .unwrap(),
            size,
        );
    }
}
criterion_group!(benches, bench_f32, bench_f64);
criterion_main!(benches);
