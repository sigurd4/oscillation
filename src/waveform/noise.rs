use num_traits::Float;
use rand::distr::{uniform::SampleUniform, Distribution, Uniform};

use super::Waveform;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct Noise;

impl<F> Waveform<F> for Noise
where
    F: Float + SampleUniform
{
    fn waveform(&self, _theta: F) -> F
    {
        let one = F::one();
        Uniform::new_inclusive(-one, one).unwrap().sample(&mut rand::rng())
    }
    fn waveform_with_dtc(&self, theta: F, duty_cycle: F) -> F
    {
        let y = self.waveform(theta);
        y.abs().powf(duty_cycle + duty_cycle).copysign(y)
    }
}

#[cfg(test)]
mod test
{
    use core::error::Error;

    use super::Noise;

    #[test]
    fn it_works() -> Result<(), Box<dyn Error>>
    {
        crate::tests::print_waveform(Noise)
    }
}