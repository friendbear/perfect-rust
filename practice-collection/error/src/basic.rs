use std::env::current_dir;
use std::fs::File;
use std::io::ErrorKind;
/// エラー種別を表す列挙型
#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
enum ValueConversion {
    Int,
    Float,
}

#[allow(dead_code)]
fn parse_string_to_num(value: String, kind: ValueConversion) {
    if kind == ValueConversion::Int {
        println!("{:?}", value.parse::<i32>());
    } else {
        println!("{:?}", value.parse::<f32>());
    }
}

#[allow(dead_code)]
fn return_io_error_kind() -> ErrorKind {
    let file_path = current_dir()
        .map(|path_buf| path_buf.join("/notfind.txt"))
        .map_err(|error| panic!("{}", error))
        .unwrap();
    let error = File::open(file_path).err();
    error.expect("unknown error").kind()
}

#[cfg(test)]
mod test_basic {
    use super::*;
    #[test]
    fn use_parse_string_to_num() {
        parse_string_to_num(String::from("123"), ValueConversion::Int); // Ok(123)
        parse_string_to_num(String::from("12.3"), ValueConversion::Float); // Ok(12.3)
        parse_string_to_num(String::from("zzz"), ValueConversion::Int); // Err(ParseIntError { kind: InvalidDigit })
        parse_string_to_num(String::from("zzz"), super::ValueConversion::Float) // Err(ParseFloatError { kind: Invalid })
    }

    #[test]
    fn use_return_io_error_kind() {
        let kind = return_io_error_kind();
        match kind {
            ErrorKind::NotFound => {}
            ErrorKind::PermissionDenied => {}
            _ => unreachable!(),
        }
    }
}
