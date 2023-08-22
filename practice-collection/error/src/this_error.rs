use thiserror::Error;
use std::num::{ParseFloatError, ParseIntError};

/// thiserrorを利用することでErrorトレイトの実装
/// #[error] アトリビュートを利用してエラーメッセージ設定ができるためDisplayトレエイトの実装が不要になる
/// 🦀 タプルにはError型ではなくFromで変換した後の型を指定している Displayトレイトのto_string結果
/// 
#[derive(Error, Debug)]
enum SampleError {
    #[error("整数変換エラー:{0}")]
    IntError(String),   
    #[error("浮動小数点変換エラー:{0}")]
    FloatError(String),
}

/// 上記の定義ではFromトレイトを実装する必要がある
impl From<ParseIntError> for SampleError {
    fn from(value: ParseIntError) -> Self {
        Self::IntError(value.to_string())
    }
}
impl From<ParseFloatError> for SampleError {
    fn from(value: ParseFloatError) -> Self {
        Self::FloatError(value.to_string())
    }
}

mod from_attribute {
    use thiserror::Error;
    use std::num::{ParseFloatError, ParseIntError};
    use num_traits::NumOps;
    use std::str::FromStr;
    #[derive(Debug, Error)]
    enum SampleError {
        #[error(transparent)] // 透過させる
        IntError(#[from] ParseIntError),
        #[error(transparent)]
        FloatError(#[from] ParseFloatError),
    }

    #[allow(dead_code)]
    fn string_to_num<T>(value: String) -> Result<T, SampleError> 
    where
        T: NumOps + FromStr,
        SampleError: From<<T as FromStr>::Err>
    {
        value.parse::<T>().map_err(SampleError::from)
    }

    #[test]
    fn use_string_to_num() {
        let result = string_to_num::<i32>(String::from("123")).unwrap();
        assert_eq!(result, 123);
        let result = string_to_num::<f32>(String::from("100.5")).unwrap();
        assert_eq!(result, 100.5);
        let result = string_to_num::<i32>(String::from("zzz")).unwrap_err();
        assert!(result.to_string().starts_with("invalid"));
        let result = string_to_num::<f32>(String::from("aaa")).unwrap_err();
        assert!(result.to_string().starts_with("invalid"));
    }
}