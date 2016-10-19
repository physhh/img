mod generic;
mod impl_core;
mod impl_gray;

pub use self::generic::*;
pub use self::impl_core::*;
pub use self::impl_gray::*;

#[test]
fn test_arithmetic() {
    use ScalarVal;

    let pixel_a = GrayVal::new(ScalarVal(1u8));
    let mut pixel_b = pixel_a + ScalarVal(1u8);
    pixel_b *= ScalarVal(1u8);
    pixel_b += pixel_a;
}

#[test]
fn test_raw_buffer_funcs() {
    use ScalarVal;

    let mut buffer = [0u8, 1, 0, 0];
    let mut pixel = GrayVal::<u8>::load_from_raw_buffer(1, 0, 4, &buffer);
    assert_eq!(pixel.intensity(), ScalarVal(1));

    pixel += ScalarVal(1);
    pixel.write_into_raw_buffer(2, 0, 4, &mut buffer);
    assert_eq!(buffer, [0, 1, 2, 0]);
}