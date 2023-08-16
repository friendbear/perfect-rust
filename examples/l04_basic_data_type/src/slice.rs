#[cfg(test)]
mod slice {
    use std::ops::Range;

    #[test]
    fn methods_1() {
        let array = [100, 101, 102, 103, 104];
        let slice: &[i32] = &array[0..];
        assert_eq!(slice.first().unwrap(), &100);
        assert_eq!(slice.last().unwrap(), &104);
        assert_eq!(slice.get(2).unwrap(), &102);
        assert!(!slice.is_empty());
        assert_eq!(slice.len(), 5);
    }

    #[test]
    fn methods_2() {
        let vec = vec!["abc", "def", "hij", "rst", "uvw", "xyz"];
        let range = Range { start: 0, end: 6 };
        let slice = &vec[range];
        for chunk in slice.chunks(3) {
            assert!(chunk.len() == 3)
        }

        let s = slice.join("/");
        assert!(s.contains("/def/"));

        let mut itr = slice.split(|s| s.eq(&"hij"));
        assert_eq!(itr.next().unwrap(), &["abc", "def"]);
        assert_eq!(itr.next().unwrap(), &["rst", "uvw", "xyz"]);
        assert_eq!(itr.next(), None);
    }
}
