use num_traits::{Float, FloatConst};

use crate::waveform::Waveform;

use super::{DirectDTC, OscillatorState, Wave};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct Direct<W>
{
    pub waveform: W
}

impl<W> From<W> for Direct<W>
{
    fn from(waveform: W) -> Self
    {
        Self {
            waveform
        }
    }
}

impl<F, W> OscillatorState<F> for Direct<W>
where
    F: Float + FloatConst,
    W: Waveform<F>
{
    type Waveform = W;

    type WithDTC = DirectDTC<F, W>;
    type WithoutDTC = Direct<W>;

    type WithWavetable<const N: usize> = Wave<F, W, N>;
    type WithoutWavetable = Direct<W>;

    type WithWaveform<WW> = Direct<WW>
    where
        WW: Waveform<F>;

    fn next(&mut self, theta: F, omega: F, rate: F) -> F
    {
        let zero = F::zero();
        let pi = F::PI();
        let nyq = pi*rate;
        if nyq <= omega
        {
            return zero
        }

        self.waveform.waveform(theta)
    }

    fn waveform(&self) -> &Self::Waveform
    {
        &self.waveform
    }
    fn waveform_mut(&mut self) -> &mut Self::Waveform
    {
        &mut self.waveform
    }

    fn with_dtc(self, duty_cycle: F) -> Self::WithDTC
    {
        DirectDTC {
            waveform: self.waveform,
            duty_cycle
        }
    }
    fn without_dtc(self) -> Self::WithoutDTC
    {
        self
    }

    fn with_wavetable<const N: usize>(self) -> Self::WithWavetable<N>
    {
        self.into()
    }
    fn without_wavetable(self) -> Self::WithoutWavetable
    {
        self
    }

    fn map_waveform<WW>(self, waveform: impl FnOnce(Self::Waveform) -> WW) -> Self::WithWaveform<WW>
    where
        WW: Waveform<F>
    {
        Direct {
            waveform: waveform(self.waveform)
        }
    }
}