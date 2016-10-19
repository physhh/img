mod generics;
mod impl_core;
mod impl_buffer;

pub use self::generics::*;
pub use self::impl_core::*;
pub use self::impl_buffer::*;

#[test]
fn test_image_buffer() {
    use {ScalarVal, Gray8U, GrayVal8U};

    let mut img = ImageBufferVal::<Gray8U>::new_with_size(2, 2);
    println!("{:?}", img.get_pixel(1, 1));

    assert_eq!(img.get_pixel(1, 1).unwrap(), GrayVal8U::new(ScalarVal(0)));
    for y in 0..2 {
        for x in 0..2 {
            let linear_idx = (y * 2 + x) as u8;
            img.set_pixel(x, y, GrayVal8U::new(ScalarVal(linear_idx)));
        }
    }
    let mut raw_buffer = [0u8; 4];
    img.write_into_raw_buffer(&mut raw_buffer);
    assert_eq!(raw_buffer, [0u8, 1, 2, 3]);
}