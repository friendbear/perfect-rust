#[allow(dead_code)]
pub fn print_row_pointer() {
    let x: i32 = 100;
    let y: &str = "ABC";

    // get row pointer
    let x_ptr: *const i32 = &x;
    let y_ptr: *const &str = &y;
    println!("ptr {:p}, {:p}", &x, x_ptr);
    println!("ptr {:p}, {:p}", &y, y_ptr);
}
#[cfg(test)]
mod test_pointer_type {

    #[test]
    fn declare_and_use_const_pointer() {
        let x: i32 = 100;
        let y: &str = "ABC";

        // get row pointer
        let x_ptr: *const i32 = &x;
        let y_ptr: *const &str = &y;
        println!("ptr {:p}, {:p}", &x, x_ptr);

        unsafe {
            //error            *x_ptr = 0;
            //error            *y_ptr = "XYZ";
            assert_eq!(*x_ptr, 100);
            assert_eq!(*y_ptr, "ABC");
        }
    }
    #[test]
    fn declare_and_use_mut_pointer() {
        let mut x: i32 = 100;
        let mut y: &str = "ABC";

        let x_ptr: *mut i32 = &mut x;
        let y_ptr: *mut &str = &mut y;
        unsafe {
            *x_ptr += 100;
            *y_ptr = "Overwrite pointer value";

            assert_eq!(*x_ptr, 200);
            assert_eq!(*y_ptr, "Overwrite pointer value")
        }
    }
}
