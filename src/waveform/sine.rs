use num_traits::{Euclid, Float, FloatConst};

use crate::Wavetable;

use super::Waveform;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct Sine;

impl<F> Waveform<F> for Sine
where
    F: Float + FloatConst + Euclid
{
    fn waveform(&self, theta: F) -> F
    {
        theta.cos()
    }
    fn waveform_with_dtc(&self, theta: F, duty_cycle: F) -> F
    {
        let half = crate::duty_cycle_default::<F>();

        if duty_cycle == half
        {
            return self.waveform(theta)
        }
        let one = F::one();
        let eps = F::from(EPS).unwrap();

        let mut a = (one - duty_cycle - duty_cycle).clamp(-one, one);
        let s = a.signum();
        a = a.abs();
        let p = (a + eps)/(a - one - eps);

        let mut numer = (-p*s*theta.cos()).exp();
        if !numer.is_finite()
        {
            numer = F::zero()
        }

        s*(p.tanh().recip() - numer/p.sinh())
    }
    fn wavetable<const N: usize>(&self) -> Option<Wavetable<F, N>>
    {
        None
    }
    fn wavetable_with_dtc<const N: usize>(&self, duty_cycle: F) -> Option<Wavetable<F, N>>
    {
        let half = crate::duty_cycle_default::<F>();

        if duty_cycle == half
        {
            return None
        }
        let zero = F::zero();
        let one = F::one();
        let eps = F::from(EPS).unwrap();

        let mut a = (one - duty_cycle - duty_cycle).clamp(-one, one);
        let s = a.signum();
        a = a.abs();
        let p = (a + eps)/(a - one - eps);

        #[repr(C)]
        struct In<const N: usize>
        {
            i0: f64,
            i_n: [f64; N]
        }

        let mut i_n = In {
            i0: 0.0,
            i_n: [0.0; N]
        };
        
        {
            let n = N.min(MAX_N);
            let x = (-p*s).to_f64().unwrap().clamp(-MAX_I_N, MAX_I_N);
            let i_n = unsafe {
                core::slice::from_raw_parts_mut(&mut i_n.i0, n + 1)
            };
            rgsl::bessel::In_array(0, n as u32, x, i_n)
                .expect("Bessel function failed.");
        }

        let g = p.sinh().recip();
        let g2 = -s*(g + g);

        Some(Wavetable::from_array(
            s*(p.tanh().recip() - g*F::from(i_n.i0).unwrap()),
            i_n.i_n.map(|i_n| (g2*F::from(i_n).unwrap(), zero))
        ))
    }
}

const MAX_N: usize = 64;
const MAX_I_N: f64 = f32::MAX_EXP as f64;
const EPS: f64 = 1.0/MAX_I_N;

#[cfg(test)]
mod test
{
    use core::error::Error;

    use super::Sine;

    #[test]
    fn it_works() -> Result<(), Box<dyn Error>>
    {
        crate::tests::print_waveform(Sine)
    }
}