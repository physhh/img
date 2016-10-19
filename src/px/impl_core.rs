use std::ops::{Add, AddAssign};
use std::ops::{Sub, SubAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Div, DivAssign};
use {ScalarVal, Pixel, PixelArithmetic};

/// Newtype which wraps [`Pixel`](trait.Pixel.html)
///
/// This type should be used in all places where you are working with actual values. This is
/// for example the case function definitions, parameters and return types. For all cases
/// where you want a type bound of a parameter you should use [`Pixel`](trait.Pixel.html).
///
/// Even though `PixelVal` does not implement [`Pixel`](trait.Pixel.html) itself, it derives
/// all functions from it. Additional to that, all functions from
/// [`PixelArithmetic`](trait.PixelArithmetic.html) are derived if it is supported.
/// Therefore it is possible to work with `PixelVal` without accessing
/// the newtype element.
///
/// # Examples
/// ```
/// use img::{ScalarVal, PixelVal, GrayVal8U};
/// let a = GrayVal8U::new(ScalarVal(21));
/// let b = GrayVal8U::new(ScalarVal(2));
/// let c = a * b;
/// assert_eq!(c.intensity(), ScalarVal(42));
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PixelVal<T>(pub T) where T: Pixel;

/// Derive all functions from [`Pixel`](trait.Pixel.html) for `PixelVal`.
impl<PixelP> PixelVal<PixelP>
    where PixelP: Pixel
{
    pub fn calc_minimum_pitch(width: u32, height: u32) -> usize {
        PixelP::calc_minimum_pitch(width, height)
    }
    pub fn calc_size_in_bytes(width: u32, height: u32, pitch: u32) -> Option<usize> {
        PixelP::calc_size_in_bytes(width, height, pitch)
    }
    pub fn load_from_raw_buffer(x: u32, y: u32, pitch: u32, buffer: &[u8]) -> Self {
        PixelVal(PixelP::load_from_raw_buffer(x, y, pitch, buffer))
    }
    pub fn write_into_raw_buffer(&self, x: u32, y: u32, pitch: u32, buffer: &mut [u8]) {
        self.0.write_into_raw_buffer(x, y, pitch, buffer)
    }
}


// implement all std ops through PixelArithmetic trait

// px <op> px
macro_rules! derive_std_op_for_px_px {
    ($op_type:ident, $op_std_func:ident, $op_px_func:ident) => (
/// Derive std operator for this newtype. This will forward all calls to the inner type.
///
/// On the inner type the corresponding function of [`PixelArithmetic`](trait.PixelArithmetic.html)
/// is called.
        impl<T> $op_type<PixelVal<T>> for PixelVal<T>
            where T: PixelArithmetic
        {
            type Output = PixelVal<T>;
            fn $op_std_func(self, rhs: Self) -> Self {
                PixelVal((self.0).$op_px_func(rhs.0))
            }
        }
    )
}
derive_std_op_for_px_px!(Add, add, add_px_px);
derive_std_op_for_px_px!(Sub, sub, sub_px_px);
derive_std_op_for_px_px!(Mul, mul, mul_px_px);
derive_std_op_for_px_px!(Div, div, div_px_px);

// px <op> sc | sc <op> px
macro_rules! derive_std_op_for_px_sc_and_sc_px {
    ($op_type:ident, $op_std_func:ident, $op_px_sc_func:ident, $op_sc_px_func:ident) => (
/// Derive std operator for this newtype. This will forward all calls to the inner type.
///
/// On the inner type the corresponding function of [`PixelArithmetic`](trait.PixelArithmetic.html)
/// is called.
        impl<T> $op_type<ScalarVal<T::ScalarT>> for PixelVal<T>
            where T: PixelArithmetic
        {
            type Output = PixelVal<T>;
            fn $op_std_func(self, rhs: ScalarVal<T::ScalarT>) -> Self::Output {
                PixelVal((self.0).$op_px_sc_func(rhs.0))
            }
        }
/// Derive std operator for this newtype. This will forward all calls to the inner type.
///
/// On the inner type the corresponding function of [`PixelArithmetic`](trait.PixelArithmetic.html)
/// is called.
        impl<T> $op_type<PixelVal<T>> for ScalarVal<T::ScalarT>
            where T: PixelArithmetic
        {
            type Output = PixelVal<T>;
            fn $op_std_func(self, rhs: PixelVal<T>) -> Self::Output {
                PixelVal((rhs.0).$op_sc_px_func(self.0))
            }
        }
    )
}
derive_std_op_for_px_sc_and_sc_px!(Add, add, add_px_sc, add_sc_px);
derive_std_op_for_px_sc_and_sc_px!(Sub, sub, sub_px_sc, sub_sc_px);
derive_std_op_for_px_sc_and_sc_px!(Mul, mul, mul_px_sc, mul_sc_px);
derive_std_op_for_px_sc_and_sc_px!(Div, div, div_px_sc, div_sc_px);

// px <assign_op> px | px <assign_op> sc
macro_rules! derive_std_assign_op_for_px_px_and_px_sc {
    ($op_type:ident, $op_std_assign_func:ident, $op_std_func:ident) => (
/// Derive std operator for this newtype. This will forward all calls to the inner type.
///
/// On the inner type the corresponding function of [`PixelArithmetic`](trait.PixelArithmetic.html)
/// is called.
        impl<T> $op_type<PixelVal<T>> for PixelVal<T>
            where T: PixelArithmetic
        {
            fn $op_std_assign_func(&mut self, rhs: PixelVal<T>) {
                *self = (*self).$op_std_func(rhs)
            }
        }
/// Derive std operator for this newtype. This will forward all calls to the inner type.
///
/// On the inner type the corresponding function of [`PixelArithmetic`](trait.PixelArithmetic.html)
/// is called.
        impl<T> $op_type<ScalarVal<T::ScalarT>> for PixelVal<T>
            where T: PixelArithmetic
        {
            fn $op_std_assign_func(&mut self, rhs: ScalarVal<T::ScalarT>) {
                *self = (*self).$op_std_func(rhs)
            }
        }
    )
}
derive_std_assign_op_for_px_px_and_px_sc!(AddAssign, add_assign, add);
derive_std_assign_op_for_px_px_and_px_sc!(SubAssign, sub_assign, sub);
derive_std_assign_op_for_px_px_and_px_sc!(MulAssign, mul_assign, mul);
derive_std_assign_op_for_px_px_and_px_sc!(DivAssign, div_assign, div);