use generic_num::num;
use num_traits::Float;

pub fn linspace<T>(start: T, stop: T, count: usize) -> impl ExactSizeIterator<Item = T>
where
    T: Float,
{
    let step = (stop - start) / num!(count.saturating_sub(1).max(1));

    (0..count).map(move |i| {
        if i == count - 1 {
            stop
        } else {
            start + num!(i) * step
        }
    })
}
