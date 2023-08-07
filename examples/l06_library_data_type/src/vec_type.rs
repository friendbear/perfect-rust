#[cfg(test)]
mod vec_type {
    use std::{cmp::Reverse, ops::Add, vec};

    #[test]
    fn instantiate() {
        let vec: Vec<i32> = Vec::with_capacity(5);
        assert!(vec.capacity() == 5);

        let vec: Vec<(i32, i32)> = vec![(10, 20), (100, 200)];
        assert!(vec.len() == 2)
    }

    #[test]
    fn add_1() {
        let mut vec: Vec<i32> = Vec::with_capacity(5);
        for value in (0..10).rev() {
            vec.push(value);
        }
        assert_eq!(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 0], &vec[..]); //compare slice

        let mut vec: Vec<(i32, i32)> = vec![(10, 20), (100, 200)];
        vec.insert(1, (1000, 2000));
        assert_eq!(&vec[1], &(1000, 2000));
    }

    #[test]
    fn get_and_change() {
        let mut vec: Vec<i32> = vec![0, 1, 2];
        assert!(vec[1].add(vec[2]) == 3);
        vec[2] = 3;
        assert!(vec[1].add(vec[2]) == 4);
    }

    #[test]
    fn remove() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let d = nums.drain(1..=2);
        let x = &d.collect::<Vec<_>>();
        assert_eq!(&[2, 3], &x[..]);

        let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut index = 0;
        while index < nums.len() {
            if nums[index] % 2 == 1 {
                nums.remove(index);
            } else {
                index += 1;
            }
        }
        assert_eq!(&[2, 4, 6, 8], &nums[..]);
    }

    #[test]
    fn sort() {
        let mut nums = vec![8, 5, 3, 7, 1, 4, 6, 2, 5, 9, 2];

        nums.sort();
        nums.dedup();
        assert_eq!(&[1, 2, 3, 4, 5, 6, 7, 8, 9], &nums[..]);

        let mut nums = vec![8, 5, 3, 7, 1, 4, 6, 2, 5, 9, 2];

        // pub fn sort_by_key<K, F>(&mut self, f: F)
        // where
        //     F: FnMut(&T) -> K,
        //     K: Ord,
        nums.sort_by_key(|&element| Reverse(element));
        nums.dedup();
        assert_eq!(&[9, 8, 7, 6, 5, 4, 3, 2, 1], &nums[..]);
    }
}
