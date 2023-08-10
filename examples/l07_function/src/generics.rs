fn add<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}
fn sub<T: std::ops::Sub<Output = T>>(x: T, y: T) -> T {
    x - y
}

pub fn use_add() {
    let r = add::<i64>(10, 20);
    println!("use_add<i64> 10 + 20 = {}", r);

    let r = add::<f64>(10.05, 20.06);
    println!("use_add<f64> 10.05 + 20.06 = {}", r);
}

pub fn use_sub() {
    let r = sub::<i64>(10, 20);
    println!("use_sub<i64> 10 + 20 = {}", r);

    let r = sub::<f32>(100.05, 20.006);
    println!("use_sub<f32> 100.05 + 20.006 = {}", r);
}
