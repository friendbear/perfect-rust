#[cfg(test)]
mod string_type {
    #[test]
    fn add() {
        let mut s = String::from("abc");
        s.push('d');
        assert!(&s == "abcd");
        s.insert(0, 'x');
        assert!(&s == "xabcd");
        s.push_str("efg");
        assert!(&s == "xabcdefg");
        s.insert_str(0, "[ ");
        s.insert_str(s.len(), " ]");
        assert!(&s == "[ xabcdefg ]");
    }
    #[test]
    fn replace() {
        let s = String::from("山田太郎, 山崎花子");
        let r = s.replace("山", "吉");
        dbg!(&r);
        assert!(r.eq("吉田太郎, 吉崎花子"));
        let r = s.replacen("山", "吉", 1);
        dbg!(&r);
        assert!(r.eq("吉田太郎, 山崎花子"));

        let mut s = s.clone();
        let offset = s.find("太").unwrap_or(s.len());
        s.replace_range(..offset, "鈴木");
        assert! {s.starts_with("鈴木")}
    }

    #[test]
    fn remove() {
        let mut s = String::from("abcdefgxyz");
        let _: () = s.clear();
        assert!(s == "");
        let mut s = String::from("abcdefgxyz");
        let d = s.drain(1..3); // 切り捨て
        let s1 = d.as_str();
        assert!(s1 == "bc");
        let p = s1.to_string().pop();
        dbg!(&p);
        assert!(p.unwrap() == 'c');
    }

    #[test]
    fn find() {
        let s = String::from("abcdeefgxyz_xyz");
        assert!(s.find("xyz").unwrap() == 8);
        assert!(s.rfind("xyz").unwrap() == 12);
    }

    #[test]
    fn matching() {
        let s = String::from("abcdeefgxyz_xyz");
        let r: Vec<_> = s.matches("xyz").collect();
        assert!(!r.is_empty());
        let r: Vec<(usize, &str)> = s.match_indices("xyz").collect();
        assert!(!r.is_empty());
        for v in r {
            assert!("xyz" == v.1 && (v.0 == 8 || v.0 == 12));
        }
    }
}
