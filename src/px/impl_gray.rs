use std::mem::{size_of, transmute, transmute_copy};
use {Scalar, ScalarVal, Pixel, PixelArithmetic, PixelVal};

/// Defines a simple grayscale pixel type.
///
/// The `BaseTypeP` type parameter specifies the data type used to store
/// the intensity value. Therefore this struct can be used to work with
/// 8bit, 16bit, ... integer values and also with 32bit, 64bit floating
/// point values.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Gray<BaseTypeP>
    where BaseTypeP: Scalar
{
    intensity: BaseTypeP,
}

impl<BaseTypeP> Pixel for Gray<BaseTypeP>
    where BaseTypeP: Scalar
{
    fn calc_minimum_pitch(width: u32, _height: u32) -> usize {
        (width as usize) * size_of::<BaseTypeP>()
    }

    fn calc_size_in_bytes(width: u32, height: u32, pitch: u32) -> Option<usize> {
        if pitch as usize >= Self::calc_minimum_pitch(width, height) {
            Some((height as usize) * (pitch as usize))
        } else {
            None
        }
    }

    fn load_from_raw_buffer(x: u32, y: u32, pitch: u32, buffer: &[u8]) -> Self {
        let start = (y * pitch) as usize + x as usize * size_of::<BaseTypeP>();
        let end = start + size_of::<BaseTypeP>();
        assert!(end <= buffer.len());
        Gray { intensity: unsafe { transmute_copy(&buffer[start]) } }
    }

    fn write_into_raw_buffer(&self, x: u32, y: u32, pitch: u32, buffer: &mut [u8]) {
        let start = (y * pitch) as usize + x as usize * size_of::<BaseTypeP>();
        let end = start + size_of::<BaseTypeP>();

        assert!(end <= buffer.len());
        let intensity: &mut BaseTypeP = unsafe { transmute(&mut buffer[start]) };
        *intensity = self.intensity;
    }
}

impl<BaseTypeP> PixelArithmetic for Gray<BaseTypeP>
    where BaseTypeP: Scalar
{
    type ScalarT = BaseTypeP;

    fn add_px_px(self, rhs: Self) -> Self {
        Gray { intensity: self.intensity + rhs.intensity }
    }
    fn sub_px_px(self, rhs: Self) -> Self {
        Gray { intensity: self.intensity - rhs.intensity }
    }
    fn mul_px_px(self, rhs: Self) -> Self {
        Gray { intensity: self.intensity * rhs.intensity }
    }
    fn div_px_px(self, rhs: Self) -> Self {
        Gray { intensity: self.intensity / rhs.intensity }
    }

    fn add_px_sc(self, rhs: Self::ScalarT) -> Self {
        Gray { intensity: self.intensity + rhs }
    }
    fn sub_px_sc(self, rhs: Self::ScalarT) -> Self {
        Gray { intensity: self.intensity - rhs }
    }
    fn mul_px_sc(self, rhs: Self::ScalarT) -> Self {
        Gray { intensity: self.intensity * rhs }
    }
    fn div_px_sc(self, rhs: Self::ScalarT) -> Self {
        Gray { intensity: self.intensity / rhs }
    }

    fn add_sc_px(self, lhs: Self::ScalarT) -> Self {
        Gray { intensity: lhs + self.intensity }
    }
    fn sub_sc_px(self, lhs: Self::ScalarT) -> Self {
        Gray { intensity: lhs - self.intensity }
    }
    fn mul_sc_px(self, lhs: Self::ScalarT) -> Self {
        Gray { intensity: lhs * self.intensity }
    }
    fn div_sc_px(self, lhs: Self::ScalarT) -> Self {
        Gray { intensity: lhs / self.intensity }
    }
}

/// Convenient abbreviation for [`Gray`](trait.Gray.html) [`PixelVal`s](struct.PixelVal.html)
pub type GrayVal<BaseTypeP> = PixelVal<Gray<BaseTypeP>>;

impl<BaseTypeP> GrayVal<BaseTypeP>
    where BaseTypeP: Scalar
{
    /// Constructs a `GrayVal` based on a given intensity value.
    pub fn new(intensity: ScalarVal<BaseTypeP>) -> GrayVal<BaseTypeP> {
        PixelVal(Gray { intensity: intensity.0 })
    }

    /// Getter for the intensity value
    pub fn intensity(&self) -> ScalarVal<BaseTypeP> {
        ScalarVal(self.0.intensity)
    }

    /// Setter for the intensity value
    pub fn set_intensity(&mut self, intensity: ScalarVal<BaseTypeP>) {
        self.0.intensity = intensity.0;
    }
}

/// Convenient abbreviation
pub type Gray8U = Gray<u8>;
/// Convenient abbreviation
pub type Gray16U = Gray<u16>;
/// Convenient abbreviation
pub type Gray32U = Gray<u32>;
/// Convenient abbreviation
pub type Gray32F = Gray<f32>;
/// Convenient abbreviation
pub type Gray64F = Gray<f64>;

/// Convenient abbreviation
pub type GrayVal8U = GrayVal<u8>;
/// Convenient abbreviation
pub type GrayVal16U = GrayVal<u16>;
/// Convenient abbreviation
pub type GrayVal32U = GrayVal<u32>;
/// Convenient abbreviation
pub type GrayVal32F = GrayVal<f32>;
/// Convenient abbreviation
pub type GrayVal64F = GrayVal<f64>;