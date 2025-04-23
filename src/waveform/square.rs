use num_traits::{Euclid, Float, FloatConst};

use crate::Wavetable;

use super::Waveform;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct Square;

impl<F> Waveform<F> for Square
where
    F: Float + FloatConst + Euclid
{
    fn waveform(&self, mut theta: F) -> F
    {
        let one = F::one();
        let pi = F::PI();
        let tau = F::TAU();

        theta = theta.rem_euclid(&tau);
        if theta < pi {-one} else {one}
    }
    fn waveform_with_dtc(&self, mut theta: F, mut duty_cycle: F) -> F
    {
        let zero = F::zero();
        let one = F::one();
        let tau = F::TAU();

        duty_cycle = duty_cycle.clamp(zero, one);
        let d = tau*duty_cycle;
        theta = theta.rem_euclid(&tau);
        if theta < d {-one} else {one}
    }

    fn wavetable<const N: usize>(&self) -> Option<Wavetable<F, N>>
    {
        let zero = F::zero();
        let pi = F::PI();

        let frac_two_pi = F::FRAC_2_PI();

        Some(Wavetable::from_fn(zero, |m| {
            let n = F::from(m + 1).unwrap();
            let g = frac_two_pi/n;

            let dn = pi*n;
            let (s, c) = dn.sin_cos();
            (
                -g*s,
                g*c - g
            )
        }))
    }

    fn wavetable_with_dtc<const N: usize>(&self, duty_cycle: F) -> Option<Wavetable<F, N>>
    {
        let zero = F::zero();
        let one = F::one();
        let pi = F::PI();
        let tau = F::TAU();

        let frac_two_pi = F::FRAC_2_PI();

        let d = tau*duty_cycle.clamp(zero, one);

        Some(Wavetable::from_fn(-(d - pi)/pi, |m| {
            let n = F::from(m + 1).unwrap();
            let g = frac_two_pi/n;

            let dn = d*n;
            let (s, c) = dn.sin_cos();
            (
                -g*s,
                g*c - g
            )
        }))
    }
}

#[cfg(test)]
mod test
{
    use core::error::Error;

    use super::Square;

    #[test]
    fn it_works() -> Result<(), Box<dyn Error>>
    {
        crate::tests::print_waveform(Square)
    }
}