#[cfg(test)]
mod test_box_type {

    #[test]
    fn instantiate() {
        let x = Box::new(100);
        let y = Box::new(200);
        assert_eq!(*x + *y, 300);

        let a = Box::new("ABCXYZ");
        let b = Box::new(String::from("XYZ"));
        assert!(a.contains(b.as_str()));
    }
}
