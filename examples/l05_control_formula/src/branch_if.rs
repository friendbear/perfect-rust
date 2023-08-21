#[cfg(test)]
mod test_branch_if {

    #[test]
    fn branch_2() {
        let num = 10;
        if num == 1 {
            println!("num is 1")
        } else if num == 2 {
            println!("num is 2")
        } else {
            
        }
    }

    #[test]
    fn branch_3() {
        let num = 10;
        let result = if num == 1 {
            "num is 1"
        } else if num == 2 {
            "num is 2"
        } else {
            "num is not 1 and 2"
        };
        assert_eq!(result, "num is not 1 and 2");
    }
}
