use crate::target::Guest;

/// ドキュメントテスト
/// 年齢が10歳でヤンペーンでなければ500を返すことを検証する
/// ```
/// use crate::testing::document_test::calc_fee_case_01;
/// let actual = calc_fee_case_01();
/// assert_eq!(500, actual);
/// ```
pub fn calc_fee_case_01() -> u32 {
    let guest = Guest::new(10, false);
    let result = guest.calc_fee().unwrap();
    result
}