#[cfg(test)]
mod test_repet_for {

    #[test]
    fn for_1() {
        let mut v = vec![];
        for i in 0..5 {
            v.push(i);
        }
        assert_eq!(&v[..], [0, 1, 2, 3, 4]);
        let mut v = vec![];
        for i in 0..=5 {
            v.push(i);
        }
        assert_eq!(&v[..], [0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn for_2() {
        let mut v = vec![];
        for i in (0..5).rev() {
            v.push(i);
        }
        assert_eq!(&v[..], [4, 3, 2, 1, 0]);

        let mut v = vec![];
        for i in (0..=5).rev() {
            v.push(i);
        }
        assert_eq!(&v[..], [5, 4, 3, 2, 1, 0]);
    }
}
