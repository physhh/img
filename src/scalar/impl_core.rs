use scalar::Scalar;
use std::ops::{Add, AddAssign};
use std::ops::{Sub, SubAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Div, DivAssign};


impl Scalar for u8 {}
impl Scalar for u16 {}
impl Scalar for u32 {}
impl Scalar for f32 {}
impl Scalar for f64 {}

/// Newtype which wraps [`Scalar`](trait.Scalar.html)
///
/// This type should be used in all places where you are working with actual values. This is
/// for example the case function definitions, parameters and return types. For all cases
/// where you want a type bound of a parameter you should use [`Scalar`](trait.Scalar.html).
///
/// Even though `ScalarVal` does not implement [`Scalar`](trait.Scalar.html) itself, it derives
/// all functions from it. Therefore it is possible to work with `ScalarVal` without accessing
/// the newtype element.
///
/// # Examples
/// ```
/// use img::ScalarVal;
/// let a = ScalarVal(21u8);
/// let b = ScalarVal(2u8);
/// let c = a * b;
/// assert_eq!(c, ScalarVal(42));
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ScalarVal<T>(pub T) where T: Scalar;

// derive ops
macro_rules! derive_std_op_for_sc {
    ($self_type:ident, $inner_trait:ident, $op_type:ident, $op_fun:ident) => (
        /// Derive std operator for this newtype. This will forward all calls to the inner type.
        impl<T> $op_type for $self_type<T>
            where T: $inner_trait
        {
            type Output = ScalarVal<<T as $op_type>::Output>;
            fn $op_fun(self, rhs: Self) -> Self::Output {
                ScalarVal((self.0).$op_fun(rhs.0))
            }
        }
    )
}
derive_std_op_for_sc!(ScalarVal, Scalar, Add, add);
derive_std_op_for_sc!(ScalarVal, Scalar, Sub, sub);
derive_std_op_for_sc!(ScalarVal, Scalar, Mul, mul);
derive_std_op_for_sc!(ScalarVal, Scalar, Div, div);


macro_rules! derive_std_assign_op_for_sc {
    ($self_type:ident, $inner_trait:ident, $op_type:ident, $op_fun:ident) => (
        /// Derive std operator for this newtype. This will forward all calls to the inner type.
        impl<T> $op_type for $self_type<T>
            where T: $inner_trait
        {
            fn $op_fun(&mut self, rhs: Self) {
                (self.0).$op_fun(rhs.0)
            }
        }
    )
}
derive_std_assign_op_for_sc!(ScalarVal, Scalar, AddAssign, add_assign);
derive_std_assign_op_for_sc!(ScalarVal, Scalar, SubAssign, sub_assign);
derive_std_assign_op_for_sc!(ScalarVal, Scalar, MulAssign, mul_assign);
derive_std_assign_op_for_sc!(ScalarVal, Scalar, DivAssign, div_assign);
