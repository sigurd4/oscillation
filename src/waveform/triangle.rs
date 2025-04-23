use num_traits::{Euclid, Float, FloatConst};

use crate::Wavetable;

use super::Waveform;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct Triangle;

impl<F> Waveform<F> for Triangle
where
    F: Float + FloatConst + Euclid
{
    fn waveform(&self, mut theta: F) -> F
    {
        let pi = F::PI();
        let tau = F::TAU();

        theta = theta.rem_euclid(&tau);
        let two_theta = theta + theta;
        (pi - (two_theta - tau).abs())/pi
    }
    fn waveform_with_dtc(&self, mut theta: F, duty_cycle: F) -> F
    {
        let zero = F::zero();
        let one = F::one();
        let tau = F::TAU();

        let d = tau*duty_cycle.clamp(zero, one);
        theta = theta.rem_euclid(&tau);
        let two_theta = theta + theta;
        if theta < d
        {
            (two_theta - d)/d
        }
        else
        {
            (tau + d - two_theta)/(tau - d)
        }
    }

    fn wavetable<const N: usize>(&self) -> Option<Wavetable<F, N>>
    {
        let zero = F::zero();
        let pi = F::PI();

        let pi_half = F::FRAC_2_PI();
        let g0 = pi_half*pi_half;

        Some(Wavetable::from_fn(zero, |m| {
            let n = F::from(m + 1).unwrap();
            let g = g0/n/n;
            let dn = pi*n;
            let (s, c) = dn.sin_cos();
            (
                g*c - g,
                g*s
            )
        }))
    }

    fn wavetable_with_dtc<const N: usize>(&self, duty_cycle: F) -> Option<Wavetable<F, N>>
    {
        let zero = F::zero();
        let one = F::one();
        let two = one + one;
        let four = two + two;
        let eps = F::epsilon();
        let tau = F::TAU();

        let frac_two_pi = F::FRAC_2_PI();

        if duty_cycle <= eps
        {
            return Some(Wavetable::from_fn(zero, |m| {
                let n = F::from(m + 1).unwrap();
                (zero, frac_two_pi/n)
            }))
        }
        if duty_cycle >= one - eps
        {
            return Some(Wavetable::from_fn(zero, |m| {
                let n = F::from(m + 1).unwrap();
                (zero, -frac_two_pi/n)
            }))
        }

        let d = tau*duty_cycle;
        let g0 = four/(tau - d)/d;

        Some(Wavetable::from_fn(zero, |m| {
            let n = F::from(m + 1).unwrap();
            let g = g0/n/n;
            let dn = d*n;
            let (s, c) = dn.sin_cos();
            (
                g*c - g,
                g*s
            )
        }))
    }
}

#[cfg(test)]
mod test
{
    use core::error::Error;

    use super::Triangle;

    #[test]
    fn it_works() -> Result<(), Box<dyn Error>>
    {
        crate::tests::print_waveform(Triangle)
    }
}