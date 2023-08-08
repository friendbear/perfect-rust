#[cfg(test)]
mod linked_list_type {
    use std::collections::LinkedList;
    #[test]
    pub fn instantiate() {
        let string_list: LinkedList<String> = LinkedList::new();
        assert!(string_list.is_empty());
        assert!(string_list.len() == 0);
    }

    #[test]
    fn add() {
        let mut string_list_a: LinkedList<String> = LinkedList::new();
        string_list_a.push_back("ABC".to_owned());
        string_list_a.push_back("DEF".to_owned());

        let mut string_list_b: LinkedList<String> = LinkedList::new();
        string_list_b.push_back("OPQ".to_owned());
        string_list_b.push_back("RST".to_owned());

        string_list_a.append(&mut string_list_b);
        string_list_a.push_front("XYZ".to_owned());
        assert!(string_list_a.front().unwrap() == "XYZ");
        assert!(string_list_a.back().unwrap() == "RST");
        assert!(string_list_a.pop_front().unwrap() == "XYZ");
        assert!(string_list_a.pop_front().unwrap() == "ABC");
        assert!(string_list_a.pop_back().unwrap() == "RST");
        assert!(string_list_a.len() == 2);
    }

    #[test]
    fn change() {
        let mut list_a = LinkedList::<i32>::new();
        list_a.extend(&[10, 20, 30]);
        let result = list_a.iter_mut().map(|x| *x * 10).collect::<Vec<i32>>();
        assert_eq!(&[100, 200, 300], &result[..]);
        match list_a.back_mut() {
            None => {}
            Some(v) => *v = 0,
        }
        assert!(list_a.back().unwrap() == &0);
    }

    #[test]
    fn remove() {
        let mut list_a = LinkedList::<i32>::new();
        list_a.extend(&[10, 20, 30]);
        let r = list_a.pop_front();
        assert!(r.unwrap() == 10);
        let r = list_a.pop_back();
        assert!(r.unwrap() == 30);

        list_a.clear();
        assert!(list_a.is_empty());
    }
}
