
pub fn ownership_move() {
    let x = String::from("XYZ");
    println!("x = {}", x);
    let y = x; // 所有権の移動
    /* 3 |     let x = String::from("XYZ");
  |         - move occurs because `x` has type `String`, which does not implement the `Copy` trait
  4 |     println!("x = {}", x);
  5 |     let y = x; // 所有権の移動
    |             - value moved here
  6 |     println!("x = {}", x);
    |                        ^ value borrowed here after move
    */
    //println!("x = {}", x);
    println!("y = {}", y);
}

pub fn ownership_reference() {
    let x = String::from("XYZ");
    println!("x = {}", x);
    let y = &x; // 所有権の参照
    println!("x = {}", x);
    println!("y = {}", y);
}

pub fn ownership_clone() {
    let x = String::from("XYZ");
    println!("x = {}", x);
    let y = x.clone(); // 所有権の参照
    println!("x = {}", x);
    println!("y = {}", y);
}

fn print_message(message: String) {
    println!("message = {}", message)
}
fn print_message_2(message: &String) {
    println!("message = {}", message)
}
pub fn ownership_function_move() {
    let x = String::from("ABC");
    /*
      |
38 |     let x = String::from("ABC");
   |         - move occurs because `x` has type `String`, which does not implement the `Copy` trait
39 |     print_message(x);
   |                   - value moved here
40 |     println!("{}", x);
   |                    ^ value borrowed here after move
     */
    print_message(x);
    // compile error println!("{}", x);
}
pub fn ownership_function_reference() {
    let x = String::from("ABC");
    let y = &x;
    print_message_2(y);
    println!("x = {}", x);
}

fn move_instance() -> String {
    let r = String::from("return string");
    r
}
pub fn ownership_function_call_return_instance_move() {
    let x = move_instance();
    println!("x = {}", x);
}