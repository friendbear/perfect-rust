/// 独自エラー型
use std::fmt::{Display, Formatter};
use std::error::Error;
use std::num::{ParseIntError, ParseFloatError};
use std::str::FromStr;
use num_traits::NumOps;
#[derive(Debug)]
enum SampleError {
    IntError(ParseIntError),
    FloatError(ParseFloatError),
}
/// std::error::Error Traitを実装
impl Error for SampleError {}

impl Display for SampleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SampleError::IntError(err) => write!(f, "error:{}, kind:{:#?}", err, err.kind()),
            SampleError::FloatError(err) => write!(f, "error:{}", err),
        }
    }
}

impl From<ParseIntError> for SampleError {
    fn from(value: ParseIntError) -> Self {
        Self::IntError(value)
    }
}
impl From<ParseFloatError> for SampleError {
    fn from(value: ParseFloatError) -> Self {
        Self::FloatError(value)
    }
}

#[allow(dead_code)]
fn parse_num<T>(value: String) -> Result<T, SampleError>
where
    T: NumOps + FromStr,
    SampleError: From<<T as FromStr>::Err>
{
    value.parse::<T>().map_err(SampleError::from)
}


#[cfg(test)]
mod test_original_error {
    use super::*;

    #[test]
    fn use_parse_num() {
        let result = parse_num::<i32>(String::from("123")).unwrap();
        assert_eq!(result, 123);
        let result = parse_num::<f32>(String::from("100.5")).unwrap();
        assert_eq!(result, 100.5);
        let result = parse_num::<i32>(String::from("zzz")).unwrap_err();
        assert!(result.to_string().starts_with("error"));
        let result = parse_num::<f32>(String::from("aaa")).unwrap_err();
        assert!(result.to_string().starts_with("error"));
    }
}