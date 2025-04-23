use num_traits::{Float, FloatConst};

use crate::{waveform::Waveform, Wavetable, WavetableView};

use super::{Direct, OscillatorState, WaveDTC};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct Wave<F, W, const N: usize>
where
    F: Float,
    W: Waveform<F>
{
    pub waveform: W,
    #[serde(skip)]
    wavetable: Option<Option<Wavetable<F, N>>>,
}

impl<F, W, const N: usize> From<W> for Wave<F, W, N>
where
    F: Float,
    W: Waveform<F>
{
    fn from(waveform: W) -> Self
    {
        Self {
            waveform,
            wavetable: None
        }
    }
}
impl<F, W, const N: usize> From<Direct<W>> for Wave<F, W, N>
where
    F: Float,
    W: Waveform<F>
{
    fn from(value: Direct<W>) -> Self
    {
        Self {
            waveform: value.waveform,
            wavetable: None
        }
    }
}

impl<F, W, const N: usize> OscillatorState<F> for Wave<F, W, N>
where
    F: Float + FloatConst,
    W: Waveform<F>
{
    type Waveform = W;

    type WithDTC = WaveDTC<F, W, N>;
    type WithoutDTC = Wave<F, W, N>;
    
    type WithoutWavetable = Direct<W>;
    type WithWavetable<const NN: usize> = Wave<F, W, NN>;

    type WithWaveform<WW> = Wave<F, WW, N>
    where
        WW: Waveform<F>;

    fn delete_cache(&mut self)
    {
        self.wavetable = None
    }
    
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
            && let Some(wavetable) = self.wavetable.get_or_insert_with(|| self.waveform.wavetable())
            && let Some(y) = wavetable.waveform(theta, up_to)
        {
            y
        }
        else
        {
            self.waveform.waveform(theta)
        }
    }

    fn duty_cycle(&self) -> F
    {
        crate::duty_cycle_default()
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
        self
    }

    fn with_wavetable<const NN: usize>(self) -> Self::WithWavetable<NN>
    {
        let Self {waveform, wavetable} = self;
        Wave {
            waveform,
            wavetable: wavetable.map(|w| w.and_then(Wavetable::truncate))
        }
    }
    fn without_wavetable(self) -> Self::WithoutWavetable
    {
        let Self {waveform, wavetable: _} = self;
        Direct {
            waveform
        }
    }
    
    fn map_waveform<WW>(self, waveform: impl FnOnce(Self::Waveform) -> WW) -> Self::WithWaveform<WW>
    where
        WW: Waveform<F>
    {
        self.without_wavetable().map_waveform(waveform).with_wavetable()
    }
}