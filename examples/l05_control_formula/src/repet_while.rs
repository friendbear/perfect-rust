#[cfg(test)]
mod repet_while {

    #[test]
    fn while_1() {
        let mut counter = 0;
        let mut v = vec![];
        while counter < 5 {
            v.push(counter);
            counter += 1;
        }
        assert_eq!(&v[..], [0, 1, 2, 3, 4]);
    }

    #[test]
    fn while_2() {
        let x = ["ABC", "ABC", "ABC", "XYZ"];
        let mut counter = 0;
        while let "ABC" = x[counter] {
            counter += 1;
        }
        assert!(counter == 3);
    }
}
