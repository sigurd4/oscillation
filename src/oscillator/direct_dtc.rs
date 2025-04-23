use num_traits::{Float, FloatConst};

use crate::waveform::Waveform;

use super::{Direct, OscillatorState, WaveDTC};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct DirectDTC<F, W>
where
    F: Float + FloatConst,
    W: Waveform<F>
{
    pub waveform: W,
    pub duty_cycle: F
}

impl<F, W> From<W> for DirectDTC<F, W>
where
    F: Float + FloatConst,
    W: Waveform<F>
{
    fn from(waveform: W) -> Self
    {
        Direct::from(waveform).into()
    }
}
impl<F, W> From<Direct<W>> for DirectDTC<F, W>
where
    F: Float + FloatConst,
    W: Waveform<F>
{
    fn from(value: Direct<W>) -> Self
    {
        value.with_dtc(crate::duty_cycle_default())
    }
}

impl<F, W> OscillatorState<F> for DirectDTC<F, W>
where
    F: Float + FloatConst,
    W: Waveform<F>
{
    type Waveform = W;

    type WithDTC = DirectDTC<F, W>;
    type WithoutDTC = Direct<W>;

    type WithWavetable<const N: usize> = WaveDTC<F, W, N>;
    type WithoutWavetable = DirectDTC<F, W>;

    type WithWaveform<WW> = DirectDTC<F, WW>
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

        self.waveform.waveform_with_dtc(theta, self.duty_cycle)
    }

    fn duty_cycle(&self) -> F
    {
        self.duty_cycle
    }

    fn waveform(&self) -> &Self::Waveform
    {
        &self.waveform
    }
    fn waveform_mut(&mut self) -> &mut Self::Waveform
    {
        &mut self.waveform
    }

    fn with_dtc(mut self, duty_cycle: F) -> Self::WithDTC
    {
        self.duty_cycle = duty_cycle;
        self
    }
    fn without_dtc(self) -> Self::WithoutDTC
    {
        let Self {waveform, duty_cycle: _} = self;
        Direct {
            waveform
        }
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
        DirectDTC {
            waveform: waveform(self.waveform),
            duty_cycle: self.duty_cycle
        }
    }
}