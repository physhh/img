use std::fmt::Debug;
use Scalar;

/// Trait which defines the minimum requirements for a pixel layout implementation.
///
/// It is important to note, that usually you want to use [`PixelVal`](struct.PixelVal.html)
/// instead of directly use this trait. Use this trait directly if you want to describe a type
/// bound. This is for example necessary if you want to define a type with a parameter,
/// which has to be a `Pixel`. To store actual values use [`PixelVal`](struct.PixelVal.html).
///
/// # Examples
/// ```
/// use img::{Pixel, PixelVal};
/// struct Foo<T: Pixel> {
///     data: PixelVal<T>,
/// };
/// ```
pub trait Pixel: Copy + Clone + Debug + PartialEq<Self> {
    /// For a given image size, this function calculates the minimum pitch in bytes.
    ///
    /// Pitch is defined as the size a row in bytes.
    /// # Examples
    ///
    /// ```
    /// use img::{Pixel, Gray8U, Gray16U, Gray32U};
    /// assert_eq!(Gray8U::calc_minimum_pitch(10, 1), 10);
    /// assert_eq!(Gray16U::calc_minimum_pitch(10, 1), 20);
    /// assert_eq!(Gray32U::calc_minimum_pitch(10, 1), 40);
    /// ```
    fn calc_minimum_pitch(width: u32, height: u32) -> usize;
    /// For a given image size, this function calculates the image size in bytes.
    ///
    /// Because the combination of `width`, `height` and `pitch` is not always valid,
    /// there are cases where the result is `None`. A common case for this to happen
    /// is when the given `pitch` is smaller than `calc_minimum_pitch(width, height)`.
    fn calc_size_in_bytes(width: u32, height: u32, pitch: u32) -> Option<usize>;
    /// Loads a `Pixel` out of a raw buffer.
    ///
    /// This is important for input output functionality.
    fn load_from_raw_buffer(x: u32, y: u32, pitch: u32, buffer: &[u8]) -> Self;
    /// Writes a `Pixel` into a raw buffer.
    ///
    /// This is important for input output functionality.
    fn write_into_raw_buffer(&self, x: u32, y: u32, pitch: u32, buffer: &mut [u8]);
}

/// Trait for [`Pixel`](trait.Pixel.html) types which can be used for arithmetic operations.
///
/// For alot of image operations it is convenient to use arithmetic operations on
/// [`Pixel`](trait.Pixel.html) types. Because this can not be supported for all implementations
/// it is a separated trait.
///
/// It is important to note, that you usually want to  use this trait directly only for type bounds.
/// This makes it possbile to define functions which require [`Pixel`](trait.Pixel.html)
/// implementations to be used in arithmetic operations.
///
/// # Examples
/// ```
/// use img::{PixelVal, PixelArithmetic};
/// fn foo<PixelT>(a: PixelVal<PixelT>, b: PixelVal<PixelT>) -> PixelVal<PixelT>
///     where PixelT: PixelArithmetic {
///     a + b
/// }
/// ```
// TODO: it might be possible to provide a simplified trait which just needs a lambda implementation
//       for pixel <op> pixel, pixel <op> scalar and scalar <op> pixel
pub trait PixelArithmetic: Pixel {
    /// This is the concrete [`Scalar`](trait.Scalar.html) implementation which can be used in
    /// arithmetic operations, with this [`Pixel`](trait.Pixel.html) type.
    ///
    /// # Examples
    /// In the example below the type of `a` is `PixelVal<Gray<u8>>` and the type of `b` is
    /// `ScalarVal<u8>`. Because `Gray<u8>::ScalarT` is `u8` it is possible to use arithmetic
    /// operations with those types.
    ///
    /// ```
    /// use img::{ScalarVal, PixelVal, GrayVal8U};
    /// let a = GrayVal8U::new(ScalarVal(21));
    /// let b = ScalarVal(2u8);
    /// let _ = a * b;
    /// ```
    type ScalarT: Scalar;

    // pixel <op> pixel

    /// Add two pixels.
    fn add_px_px(self, rhs: Self) -> Self;
    /// Subtract two pixels.
    fn sub_px_px(self, rhs: Self) -> Self;
    /// Multiply two pixels.
    fn mul_px_px(self, rhs: Self) -> Self;
    /// Divide two pixels.
    fn div_px_px(self, rhs: Self) -> Self;

    // pixel <op> scalar | scalar <op> pixel

    /// Add a pixel and a scalar
    fn add_px_sc(self, rhs: Self::ScalarT) -> Self;
    /// Subtract a pixel and a scalar
    fn sub_px_sc(self, rhs: Self::ScalarT) -> Self;
    /// Multiply a pixel and a scalar
    fn mul_px_sc(self, rhs: Self::ScalarT) -> Self;
    /// Divide a pixel and a scalar
    fn div_px_sc(self, rhs: Self::ScalarT) -> Self;

    /// Add a scalar and a pixel
    fn add_sc_px(self, lhs: Self::ScalarT) -> Self;
    /// Subtract a scalar and a pixel
    fn sub_sc_px(self, lhs: Self::ScalarT) -> Self;
    /// Multiply a scalar and a pixel
    fn mul_sc_px(self, lhs: Self::ScalarT) -> Self;
    /// Divide a scalar and a pixel
    fn div_sc_px(self, lhs: Self::ScalarT) -> Self;
}
