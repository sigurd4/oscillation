use num_traits::{Euclid, Float, FloatConst};

use crate::Wavetable;

use super::Waveform;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct Sawtooth;

impl<F> Waveform<F> for Sawtooth
where
    F: Float + FloatConst + Euclid
{
    fn waveform(&self, mut theta: F) -> F
    {
        let pi = F::PI();
        let pi_half = F::FRAC_PI_2();
        
        theta = theta.rem_euclid(&pi);
        (theta - pi_half)/pi_half
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
            (two_theta - d - tau)/(tau - d)
        }
    }

    fn wavetable<const N: usize>(&self) -> Option<Wavetable<F, N>>
    {
        let zero = F::zero();
        let pi = F::PI();

        let frac_two_pi = F::FRAC_2_PI();

        Some(Wavetable::from_fn(zero, |m| {
            let n = F::from(m + 1).unwrap();
            let g1 = frac_two_pi/n;
            let dn = pi*n;
            (
                g1*dn.sin(),
                -g1 - g1*dn.cos()
            )
        }))
    }

    fn wavetable_with_dtc<const N: usize>(&self, duty_cycle: F) -> Option<Wavetable<F, N>>
    {
        let zero = F::zero();
        let one = F::one();
        let tau = F::TAU();

        let frac_two_pi = F::FRAC_2_PI();

        let d = tau*duty_cycle.clamp(zero, one);

        Some(Wavetable::from_fn(zero, |m| {
            let n = F::from(m + 1).unwrap();
            let g1 = frac_two_pi/n;
            if d == zero || d == tau
            {
                return (zero, -g1)
            }
            let mut g2 = (d.recip() - (tau - d).recip())/n;
            if g2.is_nan()
            {
                g2 = zero
            }
            let dn = d*n;
            (
                g1*(g2*dn.cos() - g2 + dn.sin()),
                g1*(g2*dn.sin() - one - dn.cos())
            )
        }))
    }
}

#[cfg(test)]
mod test
{
    use core::error::Error;

    use super::Sawtooth;

    #[test]
    fn it_works() -> Result<(), Box<dyn Error>>
    {
        crate::tests::print_waveform(Sawtooth)
    }
}