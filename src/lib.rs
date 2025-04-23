#![cfg_attr(not(test), no_std)]
#![feature(variant_count)]
#![feature(let_chains)]
#![feature(iter_array_chunks)]
#![feature(specialization)]

use num_traits::Float;

moddef::moddef!(
    flat(pub) mod {
        wavetable
    },
    pub mod {
        oscillator,
        waveform
    },
    mod {
        plot for cfg(test),
        util
    }
);

pub fn duty_cycle_default<F>() -> F
where
    F: Float
{
    F::from(0.5).unwrap()
}

#[cfg(test)]
mod tests
{
    use core::{
        error::Error,
        f32::consts::{FRAC_PI_2, TAU},
        ops::RangeInclusive
    };

    use linspace::{Linspace, LinspaceArray};

    use crate::{
        oscillator::{Direct, Oscillator},
        waveform::Waveform
    };

    use super::*;

    const PLOT_TARGET: &str = "plots";

    #[test]
    fn it_works() {}

    fn name(type_name: &str) -> (&str, String)
    {
        let name = {
            let mut k = 0;
            let mut i = 0;
            loop
            {
                if i >= type_name.len()
                {
                    break &type_name[k..];
                }
                else if type_name[i..].starts_with("::")
                {
                    i += "::".len();
                    k = i;
                }
                else if type_name[i..].starts_with("<")
                {
                    break &type_name[k..i];
                }
                else
                {
                    i += 1;
                }
            }
        };
        let mut first = true;
        let file_name: String = name
            .chars()
            .flat_map(|c| {
                if c.is_ascii_uppercase()
                {
                    if first
                    {
                        first = false;
                        vec![c.to_ascii_lowercase()]
                    }
                    else
                    {
                        vec!['_', c.to_ascii_lowercase()]
                    }
                }
                else
                {
                    vec![c]
                }
            })
            .collect();
        (name, file_name)
    }

    pub(crate) fn print_waveform<W>(waveform: W) -> Result<(), Box<dyn Error>>
    where
        W: Waveform<f32> + Copy
    {
        const N: usize = 128;
        const M: usize = 32;
        const MM: usize = M + 1;
        const MC: usize = MM / 2;
        const MORE: f32 = FRAC_PI_2;
        const RANGE: RangeInclusive<f32> = -MORE..=(TAU + MORE);
        const RATE: f32 = N as f32 * TAU / (*RANGE.end() - *RANGE.start());
        const L: usize = 1024;
        const TIME: f32 = 1.0;

        const OMEGA: f32 = TAU;
        const PHI: f32 = *RANGE.start();

        let theta = RANGE.linspace(N);

        let dtc: [_; M] = (0.0..=1.0).linspace_array();

        let assert_finite = |x: f32| {
            assert!(x.is_finite(), "Not finite!");
            x
        };

        let y = core::array::from_fn::<_, MM, _>(|i| {
            let mut osc1 = Oscillator::new(OMEGA, PHI, Direct::from(waveform));
            let mut osc2 = osc1.with_wavetable::<L>();

            if i == MC
            {
                [
                    (0..N).map(|_| osc1.next(RATE)).map(assert_finite).collect::<Vec<_>>(),
                    (0..N).map(|_| osc2.next(RATE)).map(assert_finite).collect::<Vec<_>>()
                ]
            }
            else
            {
                let duty_cycle = dtc[i - (i >= MC) as usize];
                let mut osc1 = osc1.with_dtc(duty_cycle);
                let mut osc2 = osc2.with_dtc(duty_cycle);
                [
                    (0..N).map(|_| osc1.next(RATE)).map(assert_finite).collect::<Vec<_>>(),
                    (0..N).map(|_| osc2.next(RATE)).map(assert_finite).collect::<Vec<_>>()
                ]
            }
        });

        let (name, file_name) = name(core::any::type_name::<W>());

        plot::plot_curves_anim(
            &format!("Waveform of '{}'", name),
            &format!("{}/{}.gif", PLOT_TARGET, file_name),
            [[&theta; 2]; MM],
            y.each_ref().map(|y| y.each_ref().map(|y| y.as_slice())),
            TIME
        )
    }
}
