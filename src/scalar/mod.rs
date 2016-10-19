mod generic;
mod impl_core;

pub use self::generic::*;
pub use self::impl_core::*;

#[test]
fn test_arithmetic() {
    let a = ScalarVal(1.0);
    let b = ScalarVal(1.0);
    a + b;

    let mut d = a;
    d += a;

    a == b;
}