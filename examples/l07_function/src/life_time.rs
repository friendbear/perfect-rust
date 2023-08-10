
fn compare_len<'a>(value1: &'a String, value2: &'a String) -> &'a String {

    if value1.len() > value2.len() {
        value1
    } else {
        value2
    }
}

pub fn life_time_4() {
    let value1 = String::from("ABC");
    let value2 = String::from("DEF");

    let comp = compare_len(&value1, &value2);
    println!("result compare_len = {}", comp);
}