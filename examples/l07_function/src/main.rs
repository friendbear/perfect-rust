mod basic_function;
mod function_type;
mod generics;
mod ownership;
mod life_time;

fn main() {
    basic_functions();
    function_types();
    generics();
    ownerships();
    life_time();
}

/// 基本的な関数の定義
fn basic_functions() {
    basic_function::print_meessage_1();
    basic_function::print_meessage_2(String::from("引数付き関数"));

    let mut message = String::from("7-1-3");
    basic_function::print_meessage_3(&mut message);

    println!("{}", basic_function::print_meessage_4(&String::from("")));
    println!("{}", basic_function::print_meessage_4(&String::from("戻り値付き変数")));
    let m = basic_function::print_meessage_4(&String::from("-"));
    let _m2 = m.clone();
    println!("{}", m);
}

/// 関数型
fn function_types() {
    function_type::use_function_1();
    function_type::use_function_2();
    function_type::use_function_3();
}

/// ジェネリックとトレイと境界
fn generics() {
    generics::use_add();
    generics::use_sub();
}

/// 所有権
fn ownerships() {
    ownership::ownership_move();
    ownership::ownership_reference();
    ownership::ownership_clone();
    ownership::ownership_function_move();
    ownership::ownership_function_reference();
    ownership::ownership_function_call_return_instance_move();
}

fn life_time() {
    life_time::life_time_4();    
}