use Pixel;

/// Trait which defines the minimum requirements for an image implementation.
///
/// It is important to note, that usually you want to use [`ImageVal`](struct.ImageVal.html)
/// instead of directly use this trait. Use this trait directly if you want to describe a type
/// bound. This is for example necessary if you want to define a type with a parameter,
/// which has to be an `Image`. To store actual values use [`ImageVal`](struct.ImageVal.html).
///
/// # Examples
/// ```
/// use img::{Image, ImageVal};
/// struct Foo<T: Image> {
///     data: ImageVal<T>,
/// };
/// ```
pub trait Image: Clone {
    /// The [`Pixel`](trait.Pixel.html) type of this image.
    ///
    /// With this information we can statically enforce, that an image is only used
    /// with the correct pixel layout.
    type PixelT: Pixel;
    /// Returns the width of the image in pixels.
    fn width(&self) -> u32;
    /// Returns the height of the image in pixels.
    fn height(&self) -> u32;
    /// Returns the pitch of the image in bytes.
    fn pitch(&self) -> u32;
    /// Retrieve the pixel for a given location (`x`, `y`).
    ///
    /// If an location is accessed which is out of bound, the result will be `None`.
    fn get_pixel(&self, x: u32, y: u32) -> Option<Self::PixelT>;
    /// Stores a pixel at a location (`x`, `y`) in the `Image`.
    ///
    /// # Panics
    /// If the location is out of bounds, this function will panic.
    fn set_pixel(&mut self, x: u32, y: u32, value: Self::PixelT);
    /// Returns the memory size for the whole image in bytes.
    fn get_size_in_bytes(&self) -> usize;
    /// Loads an `Image` out of a raw buffer.
    ///
    /// This is important for input output functionality.
    fn load_from_raw_buffer(&mut self, buffer: &[u8]);
    /// Writes an `Image` into a raw buffer.
    ///
    /// This is important for input output functionality.
    fn write_into_raw_buffer(&self, buffer: &mut [u8]);
}