use core::ops::{Deref, DerefMut};

use num_traits::{Euclid, Float, FloatConst};

use crate::{waveform::Waveform, WavetableView};

moddef::moddef!(
    flat(pub) mod {
        direct_dtc,
        direct,
        wave_dtc,
        wave,
    }
);

pub trait OscillatorState<F>: Sized + From<Self::Waveform> + From<Direct<Self::Waveform>> + From<Self::WithoutDTC> + From<Self::WithoutWavetable>
where
    F: Float + FloatConst
{
    type Waveform: Waveform<F>;

    type WithDTC: OscillatorState<F, Waveform = Self::Waveform, WithDTC = Self::WithDTC> + From<Self> + From<Self::WithoutDTC> + From<DirectDTC<F, Self::Waveform>>;
    type WithoutDTC: OscillatorState<F, Waveform = Self::Waveform, WithoutDTC = Self::WithoutDTC>;

    type WithWavetable<const N: usize>: OscillatorState<F, Waveform = Self::Waveform, WithWavetable<N> = Self::WithWavetable<N>> + From<Self::WithoutWavetable> + From<Wave<F, Self::Waveform, N>>;
    type WithoutWavetable: OscillatorState<F, Waveform = Self::Waveform, WithoutWavetable = Self::WithoutWavetable>;

    type WithWaveform<W>: OscillatorState<F, Waveform = W, WithWaveform<W> = Self::WithWaveform<W>>
    where
        W: Waveform<F>;

    fn delete_cache(&mut self)
    {

    }
    
    fn next(&mut self, theta: F, omega: F, rate: F) -> F;

    fn duty_cycle(&self) -> F
    {
        crate::duty_cycle_default()
    }

    fn wavetable(&self) -> Option<WavetableView<'_, F>>
    {
        None
    }

    fn waveform(&self) -> &Self::Waveform;
    fn waveform_mut(&mut self) -> &mut Self::Waveform;

    fn with_dtc(self, duty_cycle: F) -> Self::WithDTC;
    fn without_dtc(self) -> Self::WithoutDTC;

    fn with_wavetable<const N: usize>(self) -> Self::WithWavetable<N>;
    fn without_wavetable(self) -> Self::WithoutWavetable;

    fn with_waveform<W>(self, waveform: W) -> Self::WithWaveform<W>
    where
        W: Waveform<F>
    {
        self.map_waveform(|_| waveform)
    }
    fn map_waveform<W>(self, waveform: impl FnOnce(Self::Waveform) -> W) -> Self::WithWaveform<W>
    where
        W: Waveform<F>;
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct Oscillator<F, S>
where
    F: Float + FloatConst,
    S: OscillatorState<F>
{
    pub omega: F,
    pub phi: F,
    theta: F,
    state: S
}

impl<F, S> Oscillator<F, S>
where
    F: Float + FloatConst,
    S: OscillatorState<F>
{
    pub fn new(omega: F, phi: F, mut state: S) -> Self
    {
        state.delete_cache();
        Self::new_trusted(omega, phi, state)
    }

    fn new_trusted(omega: F, phi: F, state: S) -> Self
    {
        Self {
            omega,
            phi,
            theta: F::zero(),
            state
        }
    }

    pub fn next(&mut self, rate: F) -> F
    where
        F: Euclid
    {
        let tau = F::TAU();

        self.theta = (self.theta + self.omega/rate).rem_euclid(&tau);
        self.state.next(self.theta + self.phi, self.omega, rate)
    }

    pub fn map_state<SS>(self, map: impl FnOnce(S) -> SS) -> Oscillator<F, SS>
    where
        SS: OscillatorState<F>
    {
        self.map_state_trusted(|mut state| {
            state.delete_cache();
            map(state)
        })
    }
    fn map_state_trusted<SS>(self, map: impl FnOnce(S) -> SS) -> Oscillator<F, SS>
    where
        SS: OscillatorState<F>
    {
        let Self { omega, phi, theta, state } = self;
        Oscillator {
            omega,
            phi,
            theta,
            state: map(state)
        }
    }

    pub fn with_dtc(self, duty_cycle: F) -> Oscillator<F, S::WithDTC>
    {
        self.map_state_trusted(|state| state.with_dtc(duty_cycle))
    }
    pub fn without_dtc(self) -> Oscillator<F, S::WithoutDTC>
    {
        self.map_state_trusted(|state| state.without_dtc())
    }

    pub fn with_wavetable<const N: usize>(self) -> Oscillator<F, S::WithWavetable<N>>
    {
        self.map_state_trusted(|state| state.with_wavetable())
    }
    pub fn without_wavetable(self) -> Oscillator<F, S::WithoutWavetable>
    {
        self.map_state_trusted(|state| state.without_wavetable())
    }

    pub fn with_waveform<W>(self, waveform: W) -> Oscillator<F, S::WithWaveform<W>>
    where
        W: Waveform<F>
    {
        self.map_state_trusted(|state| state.with_waveform(waveform))
    }
    pub fn map_waveform<W>(self, waveform: impl FnOnce(S::Waveform) -> W) -> Oscillator<F, S::WithWaveform<W>>
    where
        W: Waveform<F>
    {
        self.map_state_trusted(|state| state.map_waveform(waveform))
    }
}

impl<F, S> Deref for Oscillator<F, S>
where
    F: Float + FloatConst,
    S: OscillatorState<F>
{
    type Target = S;

    fn deref(&self) -> &Self::Target
    {
        &self.state
    }
}
impl<F, S> DerefMut for Oscillator<F, S>
where
    F: Float + FloatConst,
    S: OscillatorState<F>
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        self.state.delete_cache();
        &mut self.state
    }
}

#[cfg(test)]
mod test
{
    use core::{error::Error, f32::consts::TAU};

    use crate::waveform::Sawtooth;

    use super::{Oscillator, Wave};

    #[test]
    fn it_works() -> Result<(), Box<dyn Error>>
    {
        const N: usize = 16;
        const L: usize = 64;
        const RATE: f32 = 8000.0;

        let mut osc = Oscillator::new(TAU, 0.0, Wave::<_, _, L>::from(Sawtooth));

        let y = core::array::from_fn::<_, N, _>(|_| osc.next(RATE));

        println!("{:?}", y);

        Ok(())
    }
}