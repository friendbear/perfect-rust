
#[allow(dead_code)]
struct Member<'a> {
    id: u32,
    name: &'a str,
    address: &'a str,
    email: &'a str
}

impl<'a> Member<'a> {
    fn new(id: u32, name: &'a str, address: &'a str, email: &'a str) -> Self {
        Self {id, name, address, email}
    }

    fn get_name(&self) -> &str {
        self.name.clone()
    }

    fn set_name(&mut self, name: &'a str) {
        self.name = name;
    }
}

#[cfg(test)]
mod test_member {
    use super::*;

    #[test]
    fn use_method() {
        let mut customer = Member::new(100, "はしちゃん", "千葉県千葉市", "hashi@example.com");

        customer.set_name("はしちゃん🥢💙🖤");
        assert!(customer.get_name().eq("はしちゃん🥢💙🖤"));
    }
}