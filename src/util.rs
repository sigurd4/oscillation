use core::{iter::Sum, ops::{Add, Mul, MulAssign}};

use num_traits::Zero;

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
        iter.into_iter()
            .reduce(|a, b| a + b)
            .unwrap_or_else(Zero::zero)
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
        unsafe {
            core::ptr::write(self, core::ptr::read(self)*rhs)
        }
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