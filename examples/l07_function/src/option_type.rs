use std::fmt::format;

fn div(v1: i32, v2: i32) -> Option<i32> {
    if v2 == 0 {
        return None;
    }
    let r = (v1 / v2) as i32;
    Some(r)
}
fn div_to_string(v1: i32, v2: i32) -> Option<String> {
    let result = div(v1, v2)?;
    Some(result.to_string())
}
/*
fn _gpt_div(v1: i32, v2: i32) -> Option<i32> {
    v2.checked_div().and_then(|non_zero_v2| {
        (v1 / non_zero_v2).checked_add(v1 % non_zero_v2)
    })
}
*/

fn sq_then_to_string(x: u32) -> Option<String> {
    x.checked_mul(x).map(|sq| sq.to_string())
}

pub fn declare() {
    let mut _opt = None;
    _opt = Some(100);
    let mut _opt_str = Some(String::from("ABCD"));
    _opt_str = None;
}

pub fn use_div(x: i32, y: i32) {
    let r = match div(x, y) {
        Some(result) => format!("{}/{}={}", x, y, result),
        None => "Zero division".to_owned(),
    };
    println!("{}", r);
}

#[cfg(test)]
mod option_type {
    use super::*;

    #[test]
    fn method_bool() {
        assert!(div(10, 5).is_some());
        assert!(div(10, 0).is_none());
    }

    #[test]
    fn method_get() {
        assert!(div(10, 5).unwrap() == 2);
        assert!(div(10, 5).unwrap_or(-1) == 2);
        assert!(div(10, 0).unwrap_or(-1) == -1);
        assert!(div(10, 5).unwrap_or_else(|| -1) == 2);
        assert!(div(10, 0).unwrap_or_else(|| -1) == -1);
        assert!(div(10, 5).unwrap_or_default() == 2);
        assert!(div(10, 0).unwrap_or_default() == 0);
    }

    #[test]
    fn method_combinator() {
        let closure = |x: i32| x.to_string();
        // Important None.and_then(|v: U| Some(v)
        assert_eq!(None.and_then(|some_value: i32| Some(some_value)), None);
        assert_eq!(None.and_then(|some_value: String| Some(some_value)), None);
        assert_eq!(None.and_then(|some_value| Some(closure(some_value))), None);
        assert_eq!(
            Some(1).and_then(|some_value| Some(some_value.to_string())),
            Some("1".to_owned())
        );
        assert_eq!(
            Some(1).and_then(|some_value| Some(closure(some_value))),
            Some("1".to_owned())
        );

        assert_eq!(None.map(|some_value: &str| some_value.len()), None);
        assert_eq!(
            Some("ABC").map(|some_value: &str| some_value.len()),
            Some(3)
        );

        assert_eq!(None.map_or("None", |some_value| some_value), "None"); // Non Option Type
        assert_eq!(Some("ABC").map_or(0, |some_value| some_value.len()), 3);
        assert_eq!(
            Some((10, 20))
                .and_then(|v: (i32, i32)| v.0.checked_add(v.1))
                .map_or(None, |v| Some(v))
                .unwrap(),
            30
        );

        assert_eq!(None.or(Some("None")), Some("None"));
        assert_eq!(Some(1).or(Some(0)), Some(1));

        assert_eq!(None.or_else(|| Some("None")), Some("None"));
        assert_eq!(Some(1).or_else(|| Some(0)), Some(1));

        assert_eq!(
            Some(vec![("ABC", "CDF")]).map_or(Vec::<(&str, &str)>::new(), |v| v),
            vec![("ABC", "CDF")]
        );
        assert_eq!(None.map_or(Vec::<(&str, &str)>::new(), |v| v), vec![]);
    }

    #[test]
    fn method_exchange_option_to_result_type() {
        assert_eq!(
            None::<String>.ok_or("Error".to_owned()),
            Err("Error".to_owned())
        );
        assert_eq!(
            Some("".to_owned()).ok_or("Error".to_owned()),
            Ok("".to_owned())
        );

        let x = Some("foo");
        assert_eq!(x.ok_or_else(|| 0), Ok("foo"));

        let x: Option<&str> = None;
        assert_eq!(x.ok_or_else(|| 0), Err(0));

        assert_eq!(
            None::<String>.ok_or_else(|| "Error".to_owned(),),
            Err("Error".to_owned())
        );
    }

    #[test]
    fn never() {
        if let None = div_to_string(10, 0) {
            assert!(true);
        } else {
            assert!(false);
        }
    }
}
