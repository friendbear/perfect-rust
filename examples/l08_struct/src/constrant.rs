#[allow(dead_code)]
struct Customer {
    id: u32,
    name: String,
    address: String,
    email: String,
}
impl Customer {
    /// 型関連関数
    #[allow(dead_code)]
    fn new(id: u32, name: String, address: String, email: String) -> Self {
        Self {
            id,
            name,
            address,
            email,
        }
    }

    #[allow(dead_code)]
    fn get_name(&self) -> String {
        self.name.clone()
    }

    #[allow(dead_code)]
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

#[cfg(test)]
mod test_constrant {
    use super::Customer;
    #[test]
    fn use_new() {
        let customer = Customer::new(
            100,
            String::from("山田太郎"),
            String::from("東京都文京区"),
            String::from("yamada@sample.com"),
        );
        assert!(
            customer.id == 100
                && customer.name.eq("山田太郎")
                && customer.address.eq("東京都文京区")
                && customer.email.eq("yamada@sample.com")
        );
    }

    #[test]
    fn use_method() {
        let mut customer = Customer::new(
            100,
            String::from("山田太郎"),
            String::from("東京都文京区"),
            String::from("yamada@sample.com"),
        );

        customer.set_name("はしちゃん".to_owned());
        assert!(customer.get_name().eq("はしちゃん"));
    }
}
