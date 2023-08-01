fn main() {}
pub fn ignore_overflow(x: u8, y: u8) -> u8 {
    x.wrapping_add(y)
}
pub fn check_option_overflow(x: u8, y: u8) -> bool {
    x.checked_add(y).is_none()
}
pub fn check_boolean_overflow(x: u8, y: u8) -> bool {
    let (_result, overflow) = x.overflowing_add(y);
    overflow
}
#[cfg(test)]
mod operator {
    use super::*;

    #[test]
    fn overflow() {
        assert!(check_boolean_overflow(100, 200));
        assert!(check_option_overflow(100, 200));
        assert_eq!(44, ignore_overflow(100, 200));
    }
    #[test]
    fn arithmeeric() {
        use std::ops::{Add, Div, Mul, Rem, Sub};
        assert_eq!(100, 10.add(90));
        assert_eq!(5, 10.sub(5));
        assert_eq!(2, 10.div(5));
        assert_eq!(100, 10.mul(10));
        assert_eq!(2, 12.rem(10));
    }
    #[test]
    fn compound_assign_metthod() {
        use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
        let mut x: i32 = 10;
        let y: i32 = 20;
        x.add_assign(y);
        assert_eq!(30, x);

        let mut x: i32 = 10;
        x.sub_assign(y);
        assert_eq!(-10, x);

        let mut x: i32 = 10;
        x.mul_assign(y);
        assert_eq!(200, x);

        let mut x: f32 = 10.0;
        x.div_assign(y as f32);
        assert_eq!(0.5, x);

        let mut x: f32 = 45.0;
        x.rem_assign(y as f32);
        assert_eq!(5.0, x);
    }
    #[test]
    fn symbol() {
        let x: i32 = 10;
        let y: i32 = 2;

        assert!(!x.eq(&y)); // =
        assert!(x.ne(&y)); // !=
        assert!(!x.le(&y)); //<=
        assert!(x.ge(&y)); // >=
        assert!(!x.lt(&y)); // <
        assert!(x.gt(&y)); // >
    }
}
