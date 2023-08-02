#[cfg(test)]
mod str_type {

    #[test]
    fn methods_1() {
        let str_value = "ABCDEF";
        let str_slice: &[u8] = str_value.as_bytes(); // return Slice
        let mut str_chars = str_value.chars(); // return Iterator
        let str_var = "123".parse::<i32>();
        assert!(str_slice.eq(&[b'\x41', b'\x42', b'\x43', b'\x44', b'\x45', b'\x46']));
        assert_eq!(str_chars.next().unwrap(), 'A');
        assert_eq!(str_var.unwrap(), 123);
    }

    #[test]
    fn methods_2() {
        let str_value = "山田太郎";
        assert!(str_value.contains("田"));
        assert_eq!(str_value.find("田").unwrap(), 3);
        let str_value = "example.txt";
        assert!(str_value.ends_with(".txt"));
        assert!(str_value.starts_with("example"));
        assert!(!str_value.is_empty());
    }
    #[test]
    fn methods_3() {
        let str_value = "ABC\r\nDEF\r\nXYZ\r\n";
        let lines = str_value.lines().count();
        assert_eq!(lines, 3);

        let str_value = "ABCDEXYZ";
        assert_eq!(str_value.get(0..=3).unwrap(), "ABCD");
    }
    #[test]
    fn methods_4() {
        let str_value = "ABC";
        let result = str_value.repeat(3);
        assert_eq!(result, "ABCABCABC");
        let result = str_value.replace("BC", "YZ");

        assert_eq!(result, "AYZ");
    }
    #[test]
    fn methods_5() {
        let str_value = "ABC,DEF,XYZ";
        assert_eq!(str_value.to_lowercase(), String::from("abc,def,xyz"));
        assert_eq!(str_value.to_uppercase(), String::from("ABC,DEF,XYZ"));
        assert_eq!(str_value.split(',').count(), 3);
    }
}
