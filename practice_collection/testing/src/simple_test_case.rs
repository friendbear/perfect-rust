#[cfg(test)]
mod simple_test {
    use crate::target::{Guest,SampleError};
    use simple_test_case::test_case;
    // 正常系のテスト
    #[test_case(0, false, 0; "case01 age:0 campaign:false expected:0")]
    #[test_case(0, true, 0; "case02 age:0 campaign:true expected:0")]
    #[test_case(4, false, 0; "case03 age:4 campaign:false expected:0")]
    #[test_case(4, true, 0; "case04 age:4 campaign:true expected:0")]
    #[test_case(5, false, 500; "case05 age:5 campaign:false expected:500")]
    #[test_case(5, true, 450; "case06 age:5 campaign:true expected:450")]
    #[test_case(12, false, 500; "case07 age:12 campaign:false expected:500")]
    #[test_case(12, true, 450; "case08 age:12 campaign:true expected:450")]
    #[test]
    fn calc_fee_test_01_08(age: u32, campaign: bool, expected: u32) {
        let guest = Guest::new(age, campaign);
        assert_eq!(expected, guest.calc_fee().unwrap());
    }
    // Error系のテスト
    #[test_case(121, false ; "case09 age:121 campaign:false")]
    #[test_case(121, true ; "case10 age:121 campaign:true")]
    #[test]
    fn calc_fee_test_09_08(age: u32, campaign: bool) {
        let expected = SampleError::Msg(String::from("Age is incorrect."));
        let guest = Guest::new(age, campaign);
        assert_eq!(expected, guest.calc_fee().err().unwrap());
    }
}