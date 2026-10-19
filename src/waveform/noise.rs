use num_traits::Float;
use rand::{
    Rng,
    distr::{Distribution, Uniform, uniform::SampleUniform}
};

use super::Waveform;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct Noise<R>(pub R)
where
    R: Rng;

impl<R> From<R> for Noise<R>
where
    R: Rng
{
    fn from(value: R) -> Self
    {
        Self(value)
    }
}

impl<F, R> Waveform<F> for Noise<R>
where
    F: Float + SampleUniform,
    R: Rng
{
    fn waveform(&mut self, _theta: F) -> F
    {
        let one = F::one();
        Uniform::new_inclusive(-one, one).unwrap().sample(&mut self.0)
    }

    fn waveform_with_dtc(&mut self, theta: F, duty_cycle: F) -> F
    {
        let y = self.waveform(theta);
        y.abs().powf(duty_cycle + duty_cycle).copysign(y)
    }
}

#[cfg(feature = "sys")]
#[cfg(test)]
mod test
{
    use core::error::Error;

    use super::Noise;

    #[test]
    fn it_works() -> Result<(), Box<dyn Error>>
    {
        crate::tests::print_waveform(Noise(rand::rng()))
    }
}
