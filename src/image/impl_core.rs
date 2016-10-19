use std::ops::{Add, AddAssign};
use std::ops::{Sub, SubAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Div, DivAssign};
use ScalarVal;
use {PixelArithmetic, PixelVal};
use {Image, ImageBufferVal};

// TODO: The example below is currently set to 'ignore' because there is an ICE otherwise.

/// Newtype which wraps [`Image`](trait.Image.html)
///
/// This type should be used in all places where you are working with actual values. This is
/// for example the case function definitions, parameters and return types. For all cases
/// where you want a type bound of a parameter you should use [`Image`](trait.Image.html).
///
/// Even though `ImageVal` does not implement [`Image`](trait.Image.html) itself, it derives
/// all functions from it. Additional to that, it is possible to do arithmetic operations if
/// the used pixel type implement [`PixelArithmetic`](trait.PixelArithmetic.html).
/// Therefore it is possible to work with `ImageVal` without accessing
/// the newtype element.
///
/// # Examples
/// ```ignore
/// use img::{ScalarVal, Gray8U, GrayVal8U, ImageBufferVal};
///
/// // Setup image with every pixel set to the linear index
/// let mut a = ImageBufferVal::<Gray8U>::new_with_size(3, 3);
/// for y in 0..3 {
///     for x in 0..3 {
///         let linear_idx = y * 3 + x;
///         a.set_pixel(x, y, GrayVal8U::new(ScalarVal(linear_idx as u8)));
///     }
/// }
///
/// // Calculate the pixelwise square on the image directly
/// a = &a * &a;
///
/// // Check result
/// for y in 0..3 {
///     for x in 0..3 {
///         let linear_idx = y * 3 + x;
///         let linear_idx_sq = linear_idx * linear_idx;
///         assert_eq!(a.get_pixel(x, y).unwrap(),
///                     GrayVal8U::new(ScalarVal(linear_idx_sq as u8)));
///     }
/// }
/// ```
#[derive(Clone)]
pub struct ImageVal<ImageP>(pub ImageP) where ImageP: Image;

/// TODO: Impl documentation
impl<ImageP> ImageVal<ImageP>
    where ImageP: Image
{
    pub fn get_size_in_bytes(&self) -> usize {
        self.0.get_size_in_bytes()
    }
    pub fn load_from_raw_buffer(&mut self, buffer: &[u8]) {
        self.0.load_from_raw_buffer(buffer)
    }
    pub fn write_into_raw_buffer(&self, buffer: &mut [u8]) {
        self.0.write_into_raw_buffer(buffer)
    }
    pub fn width(&self) -> u32 {
        self.0.width()
    }
    pub fn height(&self) -> u32 {
        self.0.height()
    }
    pub fn pitch(&self) -> u32 {
        self.0.pitch()
    }
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<PixelVal<ImageP::PixelT>> {
        self.0.get_pixel(x, y).map(|v| PixelVal(v))
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, value: PixelVal<ImageP::PixelT>) {
        self.0.set_pixel(x, y, value.0)
    }
}

// implement all std ops through PixelArithmetic trait

// img <op> img
macro_rules! derive_std_op_for_img_img {
    ($op_type:ident, $op_std_func:ident) => (
/// TODO: Impl documentation
        impl<'a, PixelX, ImageA, ImageB> $op_type<&'a ImageVal<ImageB>> for &'a ImageVal<ImageA>
            where PixelX: PixelArithmetic,
                  ImageA: Image<PixelT = PixelX>,
                  ImageB: Image<PixelT = PixelX>
        {
            type Output = ImageBufferVal<ImageA::PixelT>;
            fn $op_std_func(self, rhs: &'a ImageVal<ImageB>) -> Self::Output {
                assert_eq!(self.width(), rhs.width());
                assert_eq!(self.height(), rhs.height());

                let mut result = Self::Output::new_with_size(self.width(), self.height());
                for y in 0..self.height() {
                    for x in 0..self.width() {
                        let new_pixel = (self.get_pixel(x, y).unwrap()).$op_std_func(rhs.get_pixel(x, y).unwrap());
                        result.set_pixel(x, y, new_pixel);
                    }
                }
                result
            }
        }
    )
}
derive_std_op_for_img_img!(Add, add);
derive_std_op_for_img_img!(Sub, sub);
derive_std_op_for_img_img!(Mul, mul);
derive_std_op_for_img_img!(Div, div);

// img <op> px | px <op> img | img <op> sc | sc <op> img
macro_rules! derive_std_op_for_img_val_and_val_img {
    ($op_type:ident, $op_std_func:ident) => (
/// TODO: Impl documentation
        impl<'a, ImageT> $op_type<PixelVal<ImageT::PixelT>> for &'a ImageVal<ImageT>
            where ImageT: Image,
                  ImageT::PixelT: PixelArithmetic
        {
            type Output = ImageBufferVal<ImageT::PixelT>;
            fn $op_std_func(self, rhs: PixelVal<ImageT::PixelT>) -> Self::Output {
                let mut result = Self::Output::new_with_size(self.width(), self.height());
                for y in 0..self.height() {
                    for x in 0..self.width() {
                        let new_pixel = (self.get_pixel(x, y).unwrap()).$op_std_func(rhs);
                        result.set_pixel(x, y, new_pixel);
                    }
                }
                result
            }
        }
/// TODO: Impl documentation
        impl<'a, ImageT> $op_type<&'a ImageVal<ImageT>> for PixelVal<ImageT::PixelT>
            where ImageT: Image,
                  ImageT::PixelT: PixelArithmetic
        {
            type Output = ImageBufferVal<ImageT::PixelT>;
            fn $op_std_func(self, rhs: &'a ImageVal<ImageT>) -> Self::Output {
                let mut result = Self::Output::new_with_size(rhs.width(), rhs.height());
                for y in 0..rhs.height() {
                    for x in 0..rhs.width() {
                        let new_pixel = (self).$op_std_func(rhs.get_pixel(x, y).unwrap());
                        result.set_pixel(x, y, new_pixel);
                    }
                }
                result
            }
        }
        /// TODO: Impl documentation
        impl<'a, ImageT> $op_type<ScalarVal<<ImageT::PixelT as PixelArithmetic>::ScalarT>> for &'a ImageVal<ImageT>
            where ImageT: Image,
                  ImageT::PixelT: PixelArithmetic
        {
            type Output = ImageBufferVal<ImageT::PixelT>;
            fn $op_std_func(self, rhs: ScalarVal<<ImageT::PixelT as PixelArithmetic>::ScalarT>) -> Self::Output {
                let mut result = Self::Output::new_with_size(self.width(), self.height());
                for y in 0..self.height() {
                    for x in 0..self.width() {
                        let new_pixel = (self.get_pixel(x, y).unwrap()).$op_std_func(rhs);
                        result.set_pixel(x, y, new_pixel);
                    }
                }
                result
            }
        }
        /// TODO: Impl documentation
        impl<'a, ImageT> $op_type<&'a ImageVal<ImageT>> for ScalarVal<<ImageT::PixelT as PixelArithmetic>::ScalarT>
            where ImageT: Image,
                  ImageT::PixelT: PixelArithmetic
        {
            type Output = ImageBufferVal<ImageT::PixelT>;
            fn $op_std_func(self, rhs: &'a ImageVal<ImageT>) -> Self::Output {
                let mut result = Self::Output::new_with_size(rhs.width(), rhs.height());
                for y in 0..rhs.height() {
                    for x in 0..rhs.width() {
                        let new_pixel = (self).$op_std_func(rhs.get_pixel(x, y).unwrap());
                        result.set_pixel(x, y, new_pixel);
                    }
                }
                result
            }
        }
    )
}
derive_std_op_for_img_val_and_val_img!(Add, add);
derive_std_op_for_img_val_and_val_img!(Sub, sub);
derive_std_op_for_img_val_and_val_img!(Mul, mul);
derive_std_op_for_img_val_and_val_img!(Div, div);

// img <assign_op> img | img <assign_op> px | img <assign_op> sc
macro_rules! derive_std_assign_op_for_img_img_and_img_val {
    ($op_type:ident, $op_std_assign_func:ident) => (
/// TODO: Impl documentation
        impl<'a, PixelX, ImageA, ImageB> $op_type<&'a ImageVal<ImageB>> for ImageVal<ImageA>
            where PixelX: PixelArithmetic,
                  ImageA: Image<PixelT = PixelX>,
                  ImageB: Image<PixelT = PixelX>
        {
            fn $op_std_assign_func(&mut self, rhs: &'a ImageVal<ImageB>) {
                assert_eq!(self.width(), rhs.width());
                assert_eq!(self.height(), rhs.height());

                for y in 0..self.height() {
                    for x in 0..self.width() {
                        let mut pixel_lhs = self.get_pixel(x, y).unwrap();
                        let pixel_rhs = rhs.get_pixel(x, y).unwrap();
                        (pixel_lhs).$op_std_assign_func(pixel_rhs);
                        self.set_pixel(x, y, pixel_lhs);
                    }
                }
            }
        }
        /// TODO: Impl documentation
        impl<ImageA> $op_type<PixelVal<ImageA::PixelT>> for ImageVal<ImageA>
            where ImageA: Image,
                  ImageA::PixelT: PixelArithmetic
        {
            fn $op_std_assign_func(&mut self, rhs: PixelVal<ImageA::PixelT>) {
                for y in 0..self.height() {
                    for x in 0..self.width() {
                        let mut pixel = self.get_pixel(x, y).unwrap();
                        (pixel).$op_std_assign_func(rhs);
                        self.set_pixel(x, y, pixel);
                    }
                }
            }
        }
        /// TODO: Impl documentation
        impl<ImageA> $op_type<ScalarVal<<ImageA::PixelT as PixelArithmetic>::ScalarT>> for ImageVal<ImageA>
            where ImageA: Image,
                  ImageA::PixelT: PixelArithmetic
        {
            fn $op_std_assign_func(&mut self, rhs: ScalarVal<<ImageA::PixelT as PixelArithmetic>::ScalarT>) {
                for y in 0..self.height() {
                    for x in 0..self.width() {
                        let mut pixel = self.get_pixel(x, y).unwrap();
                        (pixel).$op_std_assign_func(rhs);
                        self.set_pixel(x, y, pixel);
                    }
                }
            }
        }
    )
}
derive_std_assign_op_for_img_img_and_img_val!(AddAssign, add_assign);
derive_std_assign_op_for_img_img_and_img_val!(SubAssign, sub_assign);
derive_std_assign_op_for_img_img_and_img_val!(MulAssign, mul_assign);
derive_std_assign_op_for_img_img_and_img_val!(DivAssign, div_assign);