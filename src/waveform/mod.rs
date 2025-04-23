use num_traits::Float;

use crate::Wavetable;

moddef::moddef!(
    flat(pub) mod {
        waveforms,
        noise,
        rounded_triangle,
        sawtooth,
        sine,
        square,
        triangle
    }
);

pub trait Waveform<F>: Sized
where
    F: Float
{
    fn waveform(&self, theta: F) -> F;
    fn waveform_with_dtc(&self, theta: F, duty_cycle: F) -> F
    {
        let _ = duty_cycle;
        self.waveform(theta)
    }

    fn wavetable<const N: usize>(&self) -> Option<Wavetable<F, N>>
    {
        None
    }
    fn wavetable_with_dtc<const N: usize>(&self, duty_cycle: F) -> Option<Wavetable<F, N>>
    {
        let _ = duty_cycle;
        self.wavetable()
    }
}