use std::fmt::{Debug, Display, Formatter};

/// サンプルコードで利用するエラー列挙型
#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
pub enum SampleError {
    Msg(String),
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
pub struct Guest {
    age: u32,
    campaign: bool,
}
impl Display for Guest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "age:{}, canpaign:{}", self.age, self.campaign)
    }
}
impl Guest {
    #[allow(dead_code)]
    pub fn new(age: u32, campaign: bool) -> Self {
        Self { age, campaign }
    }

    /// 鑑賞金額を計算する
    #[allow(dead_code)]
    pub fn calc_fee(&self) -> Result<u32, SampleError> {
        let fee = match self.age {
            0..=4 => 0,
            5..=12 => 500,
            13..=17 => 700,
            18..=64 => 100,
            65..=120 => 600,
            _ => return Err(SampleError::Msg(String::from("Age is incorrect."))),
        };
        Ok(self.calc_campaign(fee))
    }
    /// キャンペーン中は10%引きにする
    #[allow(dead_code)]
    fn calc_campaign(&self, mut fee: u32) -> u32 {
        if self.campaign && fee != 0 {
            fee = fee * 90 / 100;
        }
        fee
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "テスト実装前"]
    #[test]
    fn calc_fee_case00() {}
    #[test]
    fn calc_fee_case01() {
        let guest = Guest::new(10, false);
        let result = guest.calc_fee().unwrap();
        assert!(result == 500, "{}", &guest);
    }
    #[test]
    fn calc_fee_case_campaign_01() {
        let guest = Guest::new(10, true);
        let result = guest.calc_fee().unwrap();
        assert!(result == 450, "{}", &guest);
    }
    #[test]
    fn calc_fee_case_02() {
        let guest = Guest::new(15, false);
        let result = guest.calc_fee().unwrap();
        assert_eq!(700, result, "{}", &guest);
    }
    #[test]
    fn calc_fee_case_campaign_02() {
        let guest = Guest::new(15, true);
        let result = guest.calc_fee().unwrap();
        assert_eq!(630, result, "{}", &guest);
    }
    #[test]
    fn calc_fee_case_03() {
        let guest = Guest::new(18, false);
        let result = guest.calc_fee().unwrap();
        assert_ne!(700, result, "{}", &guest);
    }
    #[test]
    fn calc_fee_case_campaign_03() {
        let guest = Guest::new(15, true);
        let result = guest.calc_fee().unwrap();
        assert_ne!(700, result, "{}", &guest);
    }
    #[test]
    fn calc_fee_wrong_agge() {
        let guest = Guest::new(125, false);
        let actual = guest.calc_fee().unwrap_err();
        let expected = SampleError::Msg(String::from("Age is incorrect."));
        assert_eq!(expected, actual, "{}", &actual);
    }
    #[test]
    fn calc_campagin_fee_case_01() {
        let guest = Guest::new(1000, true);
        let actual = guest.calc_campaign(1000);
        assert_eq!(900, actual);
    }
    #[test]
    #[should_panic(expected = "Age is incorrect.")]
    fn calc_fee_should_panic() {
        let guest = Guest::new(130, true);
        match guest.calc_fee() {
            Ok(_) => {}
            Err(e) => panic!("{}", e),
        }
    }
    /// Debug dbg!()
    #[test]
    fn use_debug() {
        let guest = Guest::new(0, false);
        dbg!(&guest);
        let actual = guest.calc_fee().unwrap();
        assert_eq!(0, actual);
    }
}
