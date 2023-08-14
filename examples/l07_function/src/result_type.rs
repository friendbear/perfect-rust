/// Result<T,E>
fn div(v1: i32, v2: i32) -> Result<i32, &'static str>
//where
//    T: std::ops::Div<Output = T> + PartialEq + std::fmt::Debug,
{
    if v2.eq(&0) {
        return Err("Zero division");
    }
    let r = (v1 / v2) as i32;
    Ok(r)
}

#[cfg(test)]
mod result_type {
    use super::*;

    #[test]
    fn value_setting() {
        let mut result: Result<u8, String>;
        result = Ok(b'\x01');
        assert_eq!(result.unwrap().to_string(), "1".to_string());
        result = Err("Error".to_owned());
        assert_eq!(result.unwrap_err(), "Error".to_string());
    }

    #[test]
    fn use_div() {
        let div_value = div(10, 5);
        assert_eq!(div_value.unwrap(), 2);

        let div_value = div(10, 0);
        assert_eq!(div_value.unwrap_err(), "Zero division");
    }

    #[test]
    fn method_verification() {

        assert_eq!(true, Ok::<(), ()>(()).is_ok());
        
        assert_eq!(true, Err::<(), ()>(()).is_err());
    }
    #[test]
    fn method_get() {
        assert_eq!(Ok::<(), ()>(()).unwrap(), ());
        assert_eq!(Ok::<&str, ()>("Ok").unwrap_or("Err"), "Ok");
        assert_eq!(Err::<&str, &str>("Err").unwrap_or("Err"), "Err");
        assert_eq!(Err::<(), &str>("Err").unwrap_err(), "Err");

        let div_value = div(10, 0);
        let r = div_value.unwrap_or_else(|e| {
            let error = "div error ".to_owned() + e;
            eprintln!("{}", error);
            -100
        });
        assert_eq!(r, -100);

        let div_value = div(10, 2);
        assert_eq!(div_value.unwrap_or_default(), 5);
        let div_value = div(10, 0);
        assert_eq!(div_value.unwrap_or_default(), 0);
    }

    #[test]
    fn method_combinator() {
        let r = Ok("100").and_then(|v| v.parse::<i32>());
        assert_eq!(r, Ok(100));
        let r = Ok("abc").and_then(|v| v.parse::<i32>()).is_err();
        assert!(r);

        let r = Ok::<(i32, &str), &str>((100, "str")).map(|v| v.1);
        assert_eq!(r.unwrap(), "str");

        let r: Result<&str, usize> = Err("error string").map_err(|v| v.len());
        assert_eq!(r.unwrap_err(), "error string".len());

        let r = Ok::<(i32, &str), &str>((100, "str")).map_or(0, |v| v.1.len());
        assert_eq!(r, 3);

        let r = Ok::<(i32, &str), &str>((100, "str")).map_or_else(|e| e.len(), |v| v.1.len());
        assert_eq!(r, 3);
        let r = Err::<(i32, &str), &str>("error string").map_or_else(|e| e.len(), |v| v.1.len());
        assert_eq!(r, 12);

        let r = div(10, 5).or(Err(0));
        assert_eq!(r.unwrap(), 2);
        let r = div(10, 0).or(Err("0"));
        assert_eq!(r, Err("0"));
        let r = div(10, 0).or_else(|err| Err(err));
        assert_eq!(r, Err("Zero division"));
    }
    #[test]
    fn method_exchangee_result_to_option_type() {
        if let Some(v) = div(10, 5).ok() {
            assert!(2 == v);
        }

        if let None = div(10, 0).err() {
            assert!(true);
        }
    }
}
