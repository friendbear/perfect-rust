#[cfg(test)]
mod test_repet_loop {
    use std::collections::HashMap;

    #[test]
    fn loop_1() {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut counter = 0;
        loop {
            map.insert(counter, 10);
            counter += 2;
            if map.len() >= 5 {
                break;
            }
        }
        assert!(map.get_key_value(&2).unwrap().eq(&(&2, &10)));
        assert_eq!(map.get_key_value(&1).unwrap_or((&0, &0)), (&0, &0));
    }
}
