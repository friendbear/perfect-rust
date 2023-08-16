#[cfg(test)]
mod hash_set_type {
    use std::collections::HashSet;
    #[test]
    fn instantiate() {
        let _set_a = HashSet::<i32>::new();
        let _set_b = HashSet::<i32>::with_capacity(5);
        let _set_c = HashSet::<&str>::from_iter(["abc", "dfg", "his"]);
    }

    #[test]
    fn add_and_remove() {
        let mut set_a: HashSet<i32> = vec![10, 20, 30].into_iter().collect();
        set_a.extend([40, 50, 60]);
        if set_a.insert(70) {
            assert!(true);
        } else {
            unreachable!();
        }
        if set_a.remove(&100) {
            unreachable!();
        } else {
            assert!(true);
        }

        set_a.retain(|&k| k.is_negative());
        assert!(set_a.is_empty());
    }

    #[test]
    fn get() {
        let mut set_a: HashSet<i32> = vec![10, 20, 30].into_iter().collect();
        if let Some(_v) = set_a.get(&10) {
            assert!(true);
        } else {
            unreachable!();
        }
        let b = set_a.iter().all(|v| v >= &0);
        assert!(b);
        set_a.clear();
        assert!(set_a.is_empty());
    }

    #[test]
    fn set_operation() {
        let set_a: HashSet<i32> = vec![10, 20, 30, 50, 60].into_iter().collect();
        let set_b: HashSet<i32> = vec![30, 40, 50, 70, 80].into_iter().collect();

        // aとbの和集合
        let mut x: Vec<i32> = set_a.union(&set_b).cloned().collect::<Vec<i32>>();
        x.sort();
        // TODO
        assert_eq!(vec![10, 20, 30, 40, 50, 60, 70, 80], &x[..]);
        // aとbの積集合
        let x: Vec<i32> = set_a.intersection(&set_b).cloned().collect();
        // aとbの差集合
        let x: Vec<i32> = set_a.difference(&set_b).cloned().collect();
        // aとbの対象差集合
        let x: Vec<i32> = set_a.symmetric_difference(&set_b).cloned().collect();
    }
}

