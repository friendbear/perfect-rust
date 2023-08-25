use std::fmt::{Display, Formatter, Debug};
use std::error::Error;

/// サンプルコードで利用するエラー列挙型
#[derive(Debug, Eq, PartialEq)]
enum SampleError {
    Msg(String)
}
impl Display for SampleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Msg(msg) => write!(f, "{}", msg),
        }
    }
}

/// 顧客を表す構造体
#[derive(Debug, Clone)]
struct Guest {
    age: u32,
    campaign: bool,
}
impl Display for Guest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "age:{}, canpaign:{}", self.age, self.campaign)
    }
}
impl Guest {
    pub fn new(age: u32, campaign: bool) -> Self {
        Self {age, campaign}
    }

    /// 鑑賞金額を計算する
    pub fn calc_fee(self) -> Result<u32, SampleError> {
        let fee = match self.age {
            0..=4 => 0,
            5..=12 => 500,
            13..=17 => 700,
            18..=64 => 100,
            65..=120 => 600,
            _ => return Err(SampleError::Msg(String::from("Age is incorrect.")))
        };
        Ok(self.calc_campaign(fee))
    }
    /// キャンペーン中は10%引きにする
    fn calc_campaign(&self, mut fee: u32) -> u32 {
        if self.campaign && fee != 0 {
            fee = fee * 90 / 100;
        }
        fee
    }
}

#[cfg(test)]
mod tests {
    #[ignore = "テスト実装前"]
    #[test]
    fn calc_fee_case01() {

    }
}