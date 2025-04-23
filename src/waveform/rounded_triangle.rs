use num_traits::{Euclid, Float, FloatConst};

use crate::Wavetable;

use super::{Triangle, Waveform};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct RoundedTriangle;

impl<F> Waveform<F> for RoundedTriangle
where
    F: Float + FloatConst + Euclid
{
    fn waveform(&self, theta: F) -> F
    {
        -theta.cos()
    }
    fn waveform_with_dtc(&self, theta: F, duty_cycle: F) -> F
    {
        let half_pi = F::FRAC_PI_2();
        (half_pi*Triangle.waveform_with_dtc(theta, duty_cycle)).sin()
    }
    fn wavetable<const N: usize>(&self) -> Option<Wavetable<F, N>>
    {
        None
    }
    fn wavetable_with_dtc<const N: usize>(&self, mut duty_cycle: F) -> Option<Wavetable<F, N>>
    {
        let zero = F::zero();
        let one = F::one();
        let eps = F::epsilon();
        let pi = F::PI();
        let tau = F::TAU();

        if duty_cycle + duty_cycle == one
        {
            return None
        }
        duty_cycle = duty_cycle.clamp(zero, one);
        let d = tau*duty_cycle;
        Some(Wavetable::from_fn(zero, |m| {
            let n = F::from(m + 1).unwrap();
            let f = |p| {
                let q = pi/p/n;
                ((one - q)*(one + q) + eps).recip()
            };
            let g = (f(d) - f(tau - d))/n/pi;
            if g.is_nan()
            {
                return (zero, zero)
            }
            let dn = d*n;
            let (s, c) = dn.sin_cos();
            (
                g*s,
                -g*c - g,
            )
        }))
    }
}

#[cfg(test)]
mod test
{
    use core::error::Error;

    use super::RoundedTriangle;

    #[test]
    fn it_works() -> Result<(), Box<dyn Error>>
    {
        crate::tests::print_waveform(RoundedTriangle)
    }
}