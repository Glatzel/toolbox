use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use spek::fft_backend::IFftBackend;

const SIZES: [usize; 5] = [5, 10, 15, 20, 25];

/// Benchmarks one backend's forward FFT at the given size.
macro_rules! bench_fwd {
    ($group:expr, $name:expr, $size:expr, $data:expr, $backend:expr) => {{
        $group.bench_with_input(BenchmarkId::new($name, $size), &$size, |b, _| {
            let backend = $backend;
            let mut spectrum = backend.new_spectrum();
            let mut scratch = backend.new_forward_scratch();
            b.iter(|| {
                backend.fft_unchecked(&mut $data.clone(), &mut spectrum, &mut scratch);
            })
        });
    }};
}

/// Benchmarks one backend's inverse FFT at the given size.
/// A forward transform is run once up front (outside the timed loop) to
/// populate the spectrum that `ifft_unchecked` consumes.
macro_rules! bench_inv {
    ($group:expr, $name:expr, $size:expr, $data:expr, $backend:expr) => {{
        $group.bench_with_input(BenchmarkId::new($name, $size), &$size, |b, _| {
            let backend = $backend;
            let mut spectrum = backend.new_spectrum();
            let mut fwd_scratch = backend.new_forward_scratch();
            backend.fft_unchecked(&mut $data.clone(), &mut spectrum, &mut fwd_scratch);

            let mut signal = backend.new_signal();
            let mut scratch = backend.new_inverse_scratch();
            b.iter(|| {
                backend.ifft_unchecked(&mut spectrum, &mut signal, &mut scratch);
            })
        });
    }};
}

/// Runs phastft/realfft/rustfft forward-FFT benches for every size, for a
/// given float width. `$ty` picks the element type (f32/f64), `$planner`
/// picks phastft's real-to-complex planner type.
macro_rules! fft_suite {
    ($fn_name:ident, $group_name:expr, $ty:ty, $planner:ty) => {
        fn $fn_name(c: &mut Criterion) {
            let mut group = c.benchmark_group($group_name);
            for size in SIZES {
                let data: Vec<$ty> = (0..2usize.pow(size as u32)).map(|i| i as $ty).collect();

                bench_fwd!(
                    group,
                    "phastft_fft_2^",
                    size,
                    data,
                    spek::fft_backend::phastft::PhastftBackend::<$planner>::new(data.len())
                );
                bench_fwd!(
                    group,
                    "realfft_fft_2^",
                    size,
                    data,
                    spek::fft_backend::realfft::RealfftBackend::new(data.len())
                );

                let mut planner = rustfft::FftPlanner::<$ty>::new();
                bench_fwd!(
                    group,
                    "rustfft_fft_2^",
                    size,
                    data,
                    spek::fft_backend::rustfft::RustfftBackend::new(
                        planner.plan_fft_forward(data.len()),
                        planner.plan_fft_inverse(data.len()),
                    )
                    .unwrap()
                );
            }
        }
    };
}

/// Same as `fft_suite!` but for the inverse transform.
macro_rules! ifft_suite {
    ($fn_name:ident, $group_name:expr, $ty:ty, $planner:ty) => {
        fn $fn_name(c: &mut Criterion) {
            let mut group = c.benchmark_group($group_name);
            for size in SIZES {
                let data: Vec<$ty> = (0..2usize.pow(size as u32)).map(|i| i as $ty).collect();

                bench_inv!(
                    group,
                    "phastft_ifft_2^",
                    size,
                    data,
                    spek::fft_backend::phastft::PhastftBackend::<$planner>::new(data.len())
                );
                bench_inv!(
                    group,
                    "realfft_ifft_2^",
                    size,
                    data,
                    spek::fft_backend::realfft::RealfftBackend::new(data.len())
                );

                let mut planner = rustfft::FftPlanner::<$ty>::new();
                bench_inv!(
                    group,
                    "rustfft_ifft_2^",
                    size,
                    data,
                    spek::fft_backend::rustfft::RustfftBackend::new(
                        planner.plan_fft_forward(data.len()),
                        planner.plan_fft_inverse(data.len()),
                    )
                    .unwrap()
                );
            }
        }
    };
}

fft_suite!(
    bench_fft_f32,
    "fft-f32",
    f32,
    phastft::planner::PlannerR2c32
);
ifft_suite!(
    bench_ifft_f32,
    "ifft-f32",
    f32,
    phastft::planner::PlannerR2c32
);
fft_suite!(
    bench_fft_f64,
    "fft-f64",
    f64,
    phastft::planner::PlannerR2c64
);
ifft_suite!(
    bench_ifft_f64,
    "ifft-f64",
    f64,
    phastft::planner::PlannerR2c64
);

criterion_group!(
    benches,
    bench_fft_f32,
    bench_ifft_f32,
    bench_fft_f64,
    bench_ifft_f64
);
criterion_main!(benches);
