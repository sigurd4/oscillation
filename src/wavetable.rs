use num_complex::Complex;
use num_traits::Float;

use crate::util;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Wavetable<F, const N: usize>
where
    F: Float
{
    a0: F,
    ab: [(F, F); N]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
pub struct WavetableView<'a, F>
where
    F: Float
{
    pub a0: &'a F,
    pub ab: &'a [(F, F)]
}

impl<F, const N: usize> Wavetable<F, N>
where
    F: Float
{
    pub fn from_fn(a0: F, ab: impl FnMut(usize) -> (F, F)) -> Self
    {
        Self::from_array(a0, core::array::from_fn(ab))
    }

    pub fn from_array(a0: F, ab: [(F, F); N]) -> Self
    {
        Self {
            a0,
            ab
        }
    }

    pub fn waveform(&self, theta: F, up_to: usize) -> Option<F>
    {
        let exp_1 = Complex::cis(theta);
        let mut exp_n = exp_1;

        let y = self.a0 + util::sum(self.ab[..up_to.min(N)].iter()
            .copied()
            .map(|(a, b)| {
                let y = a*exp_n.re + b*exp_n.im;
                util::mul_assign(&mut exp_n, exp_1);
                y
            }));
        if !y.is_finite()
        {
            return None
        }
        Some(y)
    }

    pub fn truncate<const M: usize>(self) -> Option<Wavetable<F, M>>
    {
        self.ab.into_iter()
            .array_chunks()
            .next()
            .map(|table| Wavetable {
                a0: self.a0,
                ab: table
            })
    }

    pub const fn view(&self) -> WavetableView<'_, F>
    {
        let Self { a0: dc, ab: table } = self;
        WavetableView { a0: dc, ab: table }
    }
}