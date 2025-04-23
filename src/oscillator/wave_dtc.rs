use num_traits::{Float, FloatConst};

use crate::{waveform::Waveform, Wavetable, WavetableView};

use super::{Direct, DirectDTC, OscillatorState, Wave};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct WaveDTC<F, W, const N: usize>
where
    F: Float + FloatConst,
    W: Waveform<F>
{
    pub waveform: W,
    pub duty_cycle: F,
    #[serde(skip)]
    wavetable: Option<Option<Wavetable<F, N>>>
}

impl<F, W, const N: usize> From<W> for WaveDTC<F, W, N>
where
    F: Float + FloatConst,
    W: Waveform<F>
{
    fn from(waveform: W) -> Self
    {
        Direct::from(waveform).into()
    }
}
impl<F, W, const N: usize> From<Direct<W>> for WaveDTC<F, W, N>
where
    F: Float + FloatConst,
    W: Waveform<F>
{
    fn from(value: Direct<W>) -> Self
    {
        DirectDTC::from(value).into()
    }
}
impl<F, W, const N: usize> From<DirectDTC<F, W>> for WaveDTC<F, W, N>
where
    F: Float + FloatConst,
    W: Waveform<F>
{
    fn from(value: DirectDTC<F, W>) -> Self
    {
        let DirectDTC {waveform, duty_cycle} = value;
        Self {
            waveform,
            duty_cycle,
            wavetable: None
        }
    }
}
impl<F, W, const N: usize> From<Wave<F, W, N>> for WaveDTC<F, W, N>
where
    F: Float + FloatConst,
    W: Waveform<F>
{
    fn from(value: Wave<F, W, N>) -> Self
    {
        value.with_dtc(crate::duty_cycle_default())
    }
}

impl<F, W, const N: usize> OscillatorState<F> for WaveDTC<F, W, N>
where
    F: Float + FloatConst,
    W: Waveform<F>
{
    type Waveform = W;

    type WithDTC = WaveDTC<F, W, N>;
    type WithoutDTC = Wave<F, W, N>;

    type WithWavetable<const NN: usize> = WaveDTC<F, W, NN>;
    type WithoutWavetable = DirectDTC<F, W>;

    type WithWaveform<WW> = WaveDTC<F, WW, N>
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

        if N != 0
            && let up_to = (nyq/omega).abs().to_usize().unwrap_or(N)
            && up_to <= N
            && let Some(wavetable) = self.wavetable.get_or_insert_with(|| self.waveform.wavetable_with_dtc(self.duty_cycle))
            && let Some(y) = wavetable.waveform(theta, up_to)
        {
            y
        }
        else
        {
            self.waveform.waveform_with_dtc(theta, self.duty_cycle)
        }
    }

    fn duty_cycle(&self) -> F
    {
        self.duty_cycle
    }

    fn wavetable(&self) -> Option<WavetableView<'_, F>>
    {
        self.wavetable.as_ref().and_then(|w| w.as_ref().map(Wavetable::view))
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
        self.without_wavetable().with_dtc(duty_cycle).with_wavetable()
    }
    fn without_dtc(self) -> Self::WithoutDTC
    {
        self.without_wavetable().without_dtc().with_wavetable()
    }

    fn with_wavetable<const NN: usize>(self) -> Self::WithWavetable<NN>
    {
        let Self {waveform, duty_cycle, wavetable} = self;
        WaveDTC {
            waveform,
            duty_cycle,
            wavetable: wavetable.map(|w| w.and_then(Wavetable::truncate))
        }
    }
    fn without_wavetable(self) -> Self::WithoutWavetable
    {
        let Self {waveform, duty_cycle, wavetable: _} = self;
        DirectDTC {
            waveform,
            duty_cycle
        }
    }

    fn map_waveform<WW>(self, waveform: impl FnOnce(Self::Waveform) -> WW) -> Self::WithWaveform<WW>
    where
        WW: Waveform<F>
    {
        self.without_wavetable().map_waveform(waveform).with_wavetable()
    }
}