#[cfg(test)]
mod hash_set_type{
    use std::collections::HashSet;
    #[test]
    fn set_operation() {
        let set_a: HashSet<i32> = vec![10, 20, 30, 50, 60].into_iter().collect();
        let set_b: HashSet<i32> = vec![30, 40, 50, 70, 80].into_iter().collect();

        // aとbの和集合
        let mut x: Vec<i32> = set_a.union(&set_b).cloned().collect();
        x.sort();
        assert_eq!(&[10, 20, 30, 40, 50, 60, 70, 80], &x[..]);
        // aとbの積集合
        let x = set_a.intersection(&set_b);
        // aとbの差集合
        let x = set_a.difference(&set_b);
        // aとbの対象差集合
        let x = set_a.symmetric_difference(&set_b);

    }
}