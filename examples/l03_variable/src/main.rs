use std::ops::AddAssign;

fn main() {
    use_constraint();
    println!("amaount = {}", calc_ammount(10000));
    println!(
        "{}",
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
            .into_iter()
            .fold(0, |sum, value| sum + value)
    );
}
const SAMPLE_NAME: &str = "Rust sample programming.";
static TAX_RATE: f32 = 0.10;
static mut TOTAL_VALUE: i32 = 0;

pub fn use_constraint() {
    const CALC_VALUE: i32 = 100;
    let result = 10 * CALC_VALUE;
    println!("calc value = {}", result);
    println!("SAMPLE_NAME = {}", SAMPLE_NAME);
    unsafe {
        for v in vec![1, 2, 3, 4, 5, 6, 7, 8, 9] {
            calc_total(v);
        }
        println!("TOTAL_VALUE = {}", TOTAL_VALUE);
    }
}

pub fn calc_total(value: i32) {
    unsafe {
        TOTAL_VALUE += value;
    }
}

pub fn calc_ammount(price: i32) -> i32 {
    let fprice = price as f32;
    let result = fprice + fprice * TAX_RATE;
    result as i32
}
