use std::vec::Vec;
use std::marker::PhantomData;
use {Pixel, Image, ImageVal};

/// Defines a buffer object which can store image data.
///
/// The type parameter `PixelP` specifies the type used in
/// [Image::PixelT](trait.Image.html#associatedtype.PixelT).
/// In addiation to the actual image data, this buffer contains some metadata about the image, like
/// size and pitch.
#[derive(Clone)]
pub struct ImageBuffer<PixelP>
    where PixelP: Pixel
{
    width: u32,
    height: u32,
    pitch: u32,
    raw_data: Vec<u8>,
    _marker: PhantomData<PixelP>,
}

/// Convenient abbreviation
pub type ImageBufferVal<PixelP> where PixelP: Pixel = ImageVal<ImageBuffer<PixelP>>;

impl<PixelP> ImageBufferVal<PixelP>
    where PixelP: Pixel
{
    /// Construct a new image buffer with the specified size and pitch.
    ///
    /// This is just a convenience function which uses the value returned from
    /// [`calc_minimum_pitch(...)`](trait.Pixel.html#tymethod.calc_minimum_pitch) as pitch.
    ///
    pub fn new_with_size(width: u32, height: u32) -> ImageBufferVal<PixelP> {
        Self::new_with_size_and_pitch(width,
                                      height,
                                      PixelP::calc_minimum_pitch(width, height) as u32)
    }

    /// Construct a new image buffer with the specified size and pitch.
    ///
    /// The raw buffer is initialized to the required size and zeroed. The pixel values therefore
    /// depend on the [`Pixel`](trait.Pixel.html) implementation.
    pub fn new_with_size_and_pitch(width: u32, height: u32, pitch: u32) -> ImageBufferVal<PixelP> {
        let size_in_bytes = PixelP::calc_size_in_bytes(width, height, pitch)
            .expect(&format!("Invalid combindation of width ({:}), height ({:}) and pitch ({:}) \
                             for this pixel type",
                             width,
                             height,
                             pitch));

        let data = vec![0; size_in_bytes];
        ImageVal(ImageBuffer {
            width: width,
            height: height,
            pitch: pitch,
            raw_data: data,
            _marker: PhantomData,
        })
    }
}

// Implement Image trait for ImageBuffer
impl<PixelP> Image for ImageBuffer<PixelP>
    where PixelP: Pixel
{
    type PixelT = PixelP;

    fn get_size_in_bytes(&self) -> usize {
        PixelP::calc_size_in_bytes(self.width, self.height, self.pitch)
            .expect(&format!("Invalid combindation of width ({:}), height ({:}) and pitch ({:}) \
                             for this pixel type - for an image ALREADY CREATED",
                             self.width,
                             self.height,
                             self.pitch))
    }
    fn load_from_raw_buffer(&mut self, buffer: &[u8]) {
        assert_eq!(self.get_size_in_bytes(), buffer.len());
        self.raw_data = Vec::from(buffer);
    }
    fn write_into_raw_buffer(&self, buffer: &mut [u8]) {
        assert_eq!(self.get_size_in_bytes(), buffer.len());
        buffer.clone_from_slice(&self.raw_data[..]);
    }
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn pitch(&self) -> u32 {
        self.pitch
    }
    fn get_pixel(&self, x: u32, y: u32) -> Option<PixelP> {
        if x < self.width || y < self.height {
            Some(PixelP::load_from_raw_buffer(x, y, self.pitch, &self.raw_data))
        } else {
            None
        }
    }
    fn set_pixel(&mut self, x: u32, y: u32, value: PixelP) {
        assert!(x < self.width || y < self.height);
        value.write_into_raw_buffer(x, y, self.pitch, &mut self.raw_data)
    }
}