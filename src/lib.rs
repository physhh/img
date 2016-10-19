// TODO: #![warn(missing_docs)]

//! TODO: Crate documentation: https://github.com/rust-lang/rust/blob/master/src/doc/book/documentation.md

mod scalar;
mod px;
mod image;

pub use scalar::{Scalar, ScalarVal};

pub use px::{Pixel, PixelArithmetic, PixelVal};
pub use px::{Gray, Gray8U, Gray16U, Gray32U, Gray32F, Gray64F};
pub use px::{GrayVal, GrayVal8U, GrayVal16U, GrayVal32U, GrayVal32F, GrayVal64F};

pub use image::{Image, ImageVal, ImageBuffer, ImageBufferVal};
