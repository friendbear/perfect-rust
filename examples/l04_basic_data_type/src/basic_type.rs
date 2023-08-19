#[cfg(test)]
mod basic_type_test {

    #[test]
    fn str_type() {
        assert_eq!("Hello Rust.", "Hello ".to_owned() + "Rust.");
    }
    #[test]
    fn tupple_method() {
        let a: (i32, i32, i32) = (100, 200, 300);
        assert_eq!(a, a.clone());
        assert!(a.eq(&(100, 200, 300)));
        assert_eq!(std::cmp::Ordering::Equal, a.cmp(&(100, 200, 300)));
        assert_eq!((200, 200, 300), a.max((200, 200, 300)));
        assert_eq!((0, 200, 300), a.min((0, 200, 300)));
    }
    #[test]
    fn use_array() {
        let mut array_a = [0; 3];
        array_a[0] = 100;
        array_a[1] = 200;
        array_a[2] = 300;

        array_a.into_iter().for_each(|v| {
            println!("{}", &v);
        });
    }
    #[test]
    fn character_type() {
        let x: char = 'x';
        assert!(x.is_ascii_alphabetic());
        assert!(!x.is_numeric());
        assert_eq!('x', x.to_ascii_lowercase());
        assert_eq!('X', x.to_ascii_uppercase());
    }
}
