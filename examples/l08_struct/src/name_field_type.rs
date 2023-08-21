#[allow(dead_code)]
struct Customer {
    id: i32,
    name: String,
    address: String,
    email: String,
}
#[allow(dead_code)]
struct Member<'a> {
    id: i32,
    name: &'a str,
    address: &'a str,
    email: &'a str,
}

#[cfg(test)]
mod test_name_field_type {
    use super::*;
    #[test]
    fn generate_instance() {
        let customer = Customer {
            id: 100,
            name: String::from("山田太郎"),
            address: String::from("東京都新宿区"),
            email: String::from("yamada@sample.com"),
        };
        assert!(
            customer.id == 100
                && customer.name.eq("山田太郎")
                && customer.address.eq("東京都新宿区")
                && customer.email.eq("yamada@sample.com")
        );

        let member = Member {
            id: 200,
            name: "山田花子",
            address: "東京都港区",
            email: "yamada@sample.com",
        };
        assert!(
            member.id == 200
                && member.name.eq("山田花子")
                && member.address.eq("東京都港区")
                && member.email.eq("yamada@sample.com")
        );
    }
}
