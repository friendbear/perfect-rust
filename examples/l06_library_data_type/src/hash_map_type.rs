/// HashMap<K,V,S=RandomState>
///
#[cfg(test)]
mod test_hash_map_type {

    use std::collections::{hash_map::RandomState, HashMap};
    #[test]
    fn instantiate() {
        let map_x: HashMap<i32, i32> = HashMap::new();
        let map_y: HashMap<i32, i32> = HashMap::with_capacity(5);
        let map_z: HashMap<i32, i32> = HashMap::from_iter([]);
        assert!(map_x.is_empty());
        assert!(map_y.is_empty());
        assert!(map_z.is_empty());
    }

    #[test]
    /// extend
    /// entry().or_insert()
    /// insert()
    fn add() {
        let mut map_x: HashMap<i32, &str> = HashMap::new();
        map_x.extend([(1, "ABC"), (2, "DEF"), (10, "XYZ")]);
        map_x.entry(10).or_insert("default");
        map_x.entry(100).or_insert("default");
        let entry: &mut &str = map_x.entry(101).or_insert("default");
        *entry = "default 101";
        dbg!(&map_x);

        let mut map_y: HashMap<usize, i32> = HashMap::from_iter([(1, 1), (2, 2)]);
        map_y.insert(4, 4);
        map_y.insert(5, 5);
    }

    #[test]
    fn get_and_change() {
        let mut map_x: HashMap<i64, &str, RandomState> =
            HashMap::from_iter([(1, "ABC"), (2, "DEF"), (10, "XYZ")]);
        assert_eq!(map_x.get(&1).unwrap(), &"ABC");
        assert!(map_x.get(&3).is_none());

        for (i, v) in map_x.iter_mut().enumerate() {
            println!("i:{}, v.0:{}, v.1:{}", i, v.0, v.1);
            match v {
                (k, value) if k == &1 => *value = "changed",
                _ => {}
            }
        }
        assert_eq!(map_x.get(&1).unwrap(), &"changed");
        assert!(map_x.contains_key(&2));
        assert_eq!(map_x.get_key_value(&2).unwrap(), (&(2_i64), &"DEF"));

        for k in map_x.keys() {
            match k {
                1 | 2 | 10 => return,
                _ => unreachable!(),
            }
        }
    }
    #[test]
    fn remove() {
        let mut map_x: HashMap<i64, &str, RandomState> =
            HashMap::from_iter([(1, "ABC"), (2, "DEF"), (10, "XYZ")]);

        assert_eq!(Some("XYZ"), map_x.remove(&10));
        assert_eq!(None, map_x.remove(&100));
        map_x.clear();
        assert!(map_x.is_empty());
    }
}
