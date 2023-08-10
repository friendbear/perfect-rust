
pub fn print_meessage_1() {
    println!("基本的な関数定義");
}

pub fn print_meessage_2(message: String) {
    println!("{}", message);
}

pub fn print_meessage_3(message: &mut String) {
    message.push_str(" push_str mutable");
    println!("{}", message);
}

pub fn print_meessage_4(message: &String) ->String {
    if message.eq("") {
        return String::from("Empty String");
    }
    println!("{}", message);
    String::from("引数を出力しました")
}