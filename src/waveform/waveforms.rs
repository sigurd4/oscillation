use num_traits::Float;

use crate as oscillation;

use crate::Wavetable;

use super::{Noise, Sawtooth, Sine, Square, Triangle};

macro_rules! waveforms {
    (
        $(#[$meta:meta])*
        $v:vis enum $group:ident: $repr:ty $(= $default:ident)?
        {
            $($w:ident),+$(,)?
        }
        $($($more:tt)+)?
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
        $v enum $group
        {
            $($w),+
        }

        $(
            impl From<$w> for $group
            {
                fn from($w: $w) -> Self
                {
                    Self::$w
                }
            }
        )*

        $(
            impl Default for $group
            {
                fn default() -> Self
                {
                    <$default as Default>::default().into()
                }
            }
        )?

        impl<F> oscillation::waveform::Waveform<F> for $group
        where
            F: Float,
            $($w: oscillation::waveform::Waveform<F>),+
        {
            fn waveform(&self, theta: F) -> F
            {
                match self
                {
                    $(Self::$w => $w.waveform(theta)),+
                }
            }
            fn waveform_with_dtc(&self, theta: F, duty_cycle: F) -> F
            {
                match self
                {
                    $(Self::$w => $w.waveform_with_dtc(theta, duty_cycle)),+
                }
            }

            fn wavetable<const N: usize>(&self) -> Option<Wavetable<F, N>>
            {
                match self
                {
                    $(Self::$w => $w.wavetable()),+
                }
            }

            fn wavetable_with_dtc<const N: usize>(&self, duty_cycle: F) -> Option<Wavetable<F, N>>
            {
                match self
                {
                    $(Self::$w => $w.wavetable_with_dtc(duty_cycle)),+
                }
            }
        }

        impl $group
        {
            #[allow(unused)]
            pub const VARIANT_COUNT: usize = core::mem::variant_count::<Self>();
            #[allow(unused)]
            pub const VARIANTS: [Self; Self::VARIANT_COUNT] = [$(Self::$w),*];
        }

        impl TryFrom<$repr> for $group
        {
            type Error = ();

            fn try_from(value: $repr) -> Result<Self, Self::Error>
            {
                Self::VARIANTS.get(value as usize).copied().ok_or(())
            }
        }
        impl From<$group> for $repr
        {
            fn from(value: $group) -> Self
            {
                value as $repr
            }
        }

        $(
            oscillation::waveforms!($($more)*);
        )?
    };
    (
        $(#[$meta:meta])*
        $v:vis enum $group:ident $(= $default:ident)?
        {
            $($w:ident),+$(,)?
        }
        $($($more:tt)+)?
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
        $v enum $group
        {
            $($w($w)),+
        }

        $(
            impl From<$w> for $group
            {
                fn from(value: $w) -> Self
                {
                    Self::$w(value)
                }
            }
        )*

        $(
            impl Default for $group
            {
                fn default() -> Self
                {
                    <$default as Default>::default().into()
                }
            }
        )?

        impl<F> oscillation::waveform::Waveform<F> for $group
        where
            F: Float,
            $($w: oscillation::waveform::Waveform<F>),+
        {
            fn waveform(&self, theta: F) -> F
            {
                match self
                {
                    $(Self::$w(w) => w.waveform(theta)),+
                }
            }
            fn waveform_with_dtc(&self, theta: F, duty_cycle: F) -> F
            {
                match self
                {
                    $(Self::$w(w) => w.waveform_with_dtc(theta, duty_cycle)),+
                }
            }

            fn wavetable<const N: usize>(&self) -> Option<Wavetable<F, N>>
            {
                match self
                {
                    $(Self::$w(w) => w.wavetable()),+
                }
            }

            fn wavetable_with_dtc<const N: usize>(&self, duty_cycle: F) -> Option<Wavetable<F, N>>
            {
                match self
                {
                    $(Self::$w(w) => w.wavetable_with_dtc(duty_cycle)),+
                }
            }
        }

        impl $group
        {
            #[allow(unused)]
            pub const VARIANT_COUNT: usize = core::mem::variant_count::<Self>();
        }

        $(
            oscillation::waveforms!($($more)*);
        )?
    };
}

waveforms!(
    pub enum MekkaWaveform: u8 = Sine
    {
        Sine,
        Triangle,
        Sawtooth,
        Square,
        Noise
    }
);

#[cfg(test)]
mod test
{
    use core::error::Error;

    use super::MekkaWaveform;

    #[test]
    fn it_works() -> Result<(), Box<dyn Error>>
    {
        crate::tests::print_waveform(MekkaWaveform::Triangle)
    }
}