#[cfg(test)]
mod str_type {

    #[test]
    fn methods_1() {
        let str_value = "ABCDEF";
        let str_slice: &[u8] = str_value.as_bytes(); // return Slice
        let mut str_chars  = str_value.chars(); // return Iterator
        let str_var  = "123".parse::<i32>();
        assert!(str_slice.eq(&[b'\x41', b'\x42', b'\x43', b'\x44', b'\x45', b'\x46']));
        assert_eq!(str_chars.next().unwrap(), 'A');
        assert_eq!(str_var.unwrap(), 123);
    }
}