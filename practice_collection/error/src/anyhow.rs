use num_traits::NumOps;
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
enum SampleError {
    #[error(transparent)]
    IntError(#[from] ParseIntError),
    #[error(transparent)]
    FloatError(#[from] ParseFloatError),
}

// Result型のエイリアス SampleResult
type SampleResult<T> = anyhow::Result<T, anyhow::Error>;

#[allow(dead_code)]
fn parse_string_to_num<T>(value: String) -> SampleResult<T>
where
    T: NumOps + FromStr,
    SampleError: From<<T as FromStr>::Err>,
{
    value.parse::<T>().map_err(|err| {
        let content = format!("指定された{}は変換できませんでした", value);
        let error = SampleError::from(err);
        anyhow::Error::new(error).context(content)
    })
}

#[cfg(test)]
mod test_anyhow {
    use super::*;
    #[test]
    fn use_parse_string_to_num() {
        let result = parse_string_to_num::<i32>(String::from("value"));

        assert!(result.is_err());
        println!("{:?}", result.err().unwrap().downcast::<&str>());
    }
}
