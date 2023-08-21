#[cfg(test)]
mod test_branch_match {

    #[test]
    fn branch_1() {
        let x = 10;
        match x {
            1 => unreachable!(),
            2 => unreachable!(),
            _ => {},
        }
    }
    #[test]
    fn branch_2() {
        let x = "山田太郎";
        match x {
            "山田太郎" => {},
            "鈴木花子" => unreachable!(),
            _ => unreachable!(),
        }
    }

    #[test]
    /// match-let
    fn branch_3() {
        let calc = |x: i32| x * 10;
        let y = 3;
        let result = match y {
            1 => calc(10),
            2 => calc(20),
            3 => calc(30),
            _ => calc(0),
        };
        assert!(result == 300);
    }
    #[test]
    /// Range and OR
    fn branch_4() {
        let calc = |x: i32| x * 10;
        let value = 30;
        let result = match value {
            1..=3 => calc(10),
            4..=6 => calc(20),
            7..=9 => calc(30),
            10 | 20 | 30 => calc(40),
            21..=25 | 31..=35 => calc(50),
            _ => calc(0),
        };
        assert!(result == 400);
    }
    #[test]
    /// guard
    fn branch_5() {
        let value = (10, 25);
        let result = match value {
            (x, y) if x == 0 && y == 0 => "zero",
            (x, y) if x % 2 == 0 && y % 2 == 0 => "even number",
            (x, y) if x % 2 == 1 && y % 2 == 1 => "odd number",
            _ => "None",
        };
        assert!(result == "None");
    }
}
