use std::fmt::Debug;
use std::ops::{Add, AddAssign};
use std::ops::{Sub, SubAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Div, DivAssign};

/// Trait which defines the minimum requirements for a numeric type in the context of this crate.
///
/// It is important to note, that usually you want to use [`ScalarVal`](struct.ScalarVal.html)
/// instead of directly use this trait. Use this trait directly if you want to describe a type
/// bound. This is for example necessary if you want to define a type with a parameter,
/// which has to be a `Scalar`. To store actual values use [`ScalarVal`](struct.ScalarVal.html).
///
/// # Examples
/// ```
/// use img::{Scalar, ScalarVal};
/// struct Foo<T: Scalar> {
///     data: ScalarVal<T>,
/// };
/// ```
pub trait Scalar:
    Copy + Clone + Debug
    + PartialEq<Self>
    + Add<Self, Output = Self> + AddAssign<Self>
    + Sub<Self, Output = Self> + SubAssign<Self>
    + Mul<Self, Output = Self> + MulAssign<Self>
    + Div<Self, Output = Self> + DivAssign<Self> {}
