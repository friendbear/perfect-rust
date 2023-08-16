#[cfg(test)]
mod data_type {

    #[test]
    fn methods_1() {
        let x: i64 = -10_i64;
        assert_eq!(10, x.abs());
        assert_eq!(-1, x.signum());
        assert_eq!(-1000, x.pow(3));
        assert!(!x.is_positive());
        assert!(x.is_negative());
        assert_eq!(x, x.clone());
        assert_eq!(String::from("-10"), x.to_string());
    }

    #[test]
    fn floating_type() {
        let x: f64 = 10.5_f64;
        assert_eq!(11_f64, x.ceil());
        assert_eq!(10_f64, x.floor());
    }
    #[test]
    fn boolean_method() {
        assert_eq!(true.then(|| "true"), Some("true"));
    }

    #[test]
    fn char_method() {
        let x: char = 'x';
        assert!(x.is_ascii_alphabetic());
        assert!(!x.is_numeric());
        assert_eq!(x.to_ascii_lowercase(), 'x');
        assert_eq!(x.to_ascii_uppercase(), 'X');
    }

    #[test]
    fn array_declare() {
        let array_a = [1, 2, 3];
        let array_b: [i32; 3] = [10, 20, 30];
        let array_c = [0; 3];

        assert!(array_a.eq(&[1, 2, 3]));
        assert!(array_b.eq(&[10, 20, 30]));
        assert!(array_c.eq(&[0, 0, 0]));
    }
    #[test]
    fn array_method() {
        let mut array_a: [i32; 5] = [2, 3, 5, 4, 1];

        assert!(!array_a.is_empty());
        assert!(array_a.contains(&5));
        assert_eq!(array_a.first(), Some(&2));
        assert_eq!(array_a.last(), Some(&1));

        array_a.sort();
        assert_eq!(array_a, [1, 2, 3, 4, 5]);

        array_a.reverse();
        assert_eq!(array_a, [5, 4, 3, 2, 1]);
    }
    #[test]
    fn multidimentional() {
        let array_2 = [[0; 5]; 3];
        let array_3 = [[[0; 5]; 3]; 2];
        for array in array_2 {
            assert!(array.eq(&[0, 0, 0, 0, 0]));
        }

        for array_2 in array_3 {
            for array in array_2 {
                assert!(array.eq(&[0, 0, 0, 0, 0]));
            }
        }
    }
    #[test]
    /// tuple type
    fn tuple_type() {
        let x: (i32, &str, bool) = (100, "Hello", true);

        let (_a, _b, c) = (100, "Hello", true);

        assert!(x.2);
        assert!(c);
    }
}
