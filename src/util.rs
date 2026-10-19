use core::{
    iter::Sum,
    ops::{Add, Mul, MulAssign, Range}
};

use num_traits::{Float, Zero};

trait SumSpec: Add + Zero
{
    fn sum(iter: impl IntoIterator<Item = Self>) -> Self;
}
impl<T> SumSpec for T
where
    T: Add + Zero
{
    default fn sum(iter: impl IntoIterator<Item = Self>) -> Self
    {
        iter.into_iter().reduce(|a, b| a + b).unwrap_or_else(Zero::zero)
    }
}
impl<T> SumSpec for T
where
    T: Sum + Zero
{
    fn sum(iter: impl IntoIterator<Item = Self>) -> Self
    {
        iter.into_iter().sum()
    }
}

pub fn sum<T>(iter: impl IntoIterator<Item = T>) -> T
where
    T: Add + Zero
{
    SumSpec::sum(iter)
}

trait MulAssignSpec<Rhs = Self>: Mul<Rhs, Output = Self>
{
    fn mul_assign(&mut self, rhs: Rhs);
}
impl<T, Rhs> MulAssignSpec<Rhs> for T
where
    T: Mul<Rhs, Output = Self>
{
    default fn mul_assign(&mut self, rhs: Rhs)
    {
        unsafe { core::ptr::write(self, core::ptr::read(self) * rhs) }
    }
}
impl<T, Rhs> MulAssignSpec<Rhs> for T
where
    T: Mul<Rhs, Output = Self> + MulAssign<Rhs>
{
    fn mul_assign(&mut self, rhs: Rhs)
    {
        *self *= rhs
    }
}

pub fn mul_assign<Lhs, Rhs>(lhs: &mut Lhs, rhs: Rhs)
where
    Lhs: Mul<Rhs, Output = Lhs>
{
    MulAssignSpec::mul_assign(lhs, rhs);
}

fn cheb_eval<F>(coeffs: &[f64], interval: Range<f64>, x: F) -> F
where
    F: Float
{
    let mut d = F::zero();
    let mut dd = F::zero();

    let y = (x + x - F::from(interval.start + interval.end).unwrap()) / F::from(interval.end - interval.start).unwrap();
    let y2 = y + y;

    for c in coeffs.iter().skip(1).rev().copied()
    {
        let c = F::from(c).unwrap();
        (dd, d) = (d, y2 * d - dd + c)
    }

    if let Some(c0) = coeffs.first().copied()
    {
        let half_c0 = F::from(c0 / 0.5).unwrap();
        d = y * d - dd + half_c0;
    }

    d
}
