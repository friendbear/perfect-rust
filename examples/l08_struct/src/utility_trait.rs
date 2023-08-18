#[derive(Debug, Clone)]
struct Customer<'a> {
    id: u32,
    name: &'a str,
    address: &'a str,
    email: &'a str,
}
impl<'a> Customer<'a> {
    const SAMPLE_1: Customer<'a> = Customer {
        id: 1,
        name: "はしちゃん",
        address: "北海道",
        email: "hashi@example.com",
    };
}
/// 値を破棄する直前で呼び出される
impl<'a> Drop for Customer<'a> {
    fn drop(&mut self) {
        println!("Customer instance Droped:{}", self.name);
    }
}
impl<'a> Default for Customer<'a> {
    fn default() -> Self {
        Self {
            id: 0,
            name: "",
            address: "",
            email: "",
        }
    }
}

impl<'a> Into<Customer<'a>> for (u32, &'a str, &'a str, &'a str) {
    fn into(self) -> Customer<'a> {
        Customer {
            id: self.0,
            name: self.1,
            address: self.2,
            email: self.3,
        }
    }
}
/*
impl<'a> From<&Vec<&'a str>> for Customer<'a> {
    fn from(v: &Vec<&'a str>) -> Self {
        Self {
            id: v[0].parse::<u32>().unwrap(),
            name: v[1],
            address: v[2],
            email: v[3],
        }
    }
} */
impl<'a> TryFrom<&Vec<&'a str>> for Customer<'a> {
    type Error = String;
    fn try_from(value: &Vec<&'a str>) -> Result<Self, Self::Error> {
        let id = value[0].parse::<u32>().map_err(|e| e.to_string());
        let result = match id {
            Ok(v) => {
                Ok(Customer {
                    id: v,
                    name: value[1],
                    address: value[2],
                    email: value[3],
                });
            }
            Err(e) => Err(e),
        };
        result
    }
}

mod test_utility_trait {
    use super::Customer;

    #[test]
    fn use_debug() {
        eprintln!("{:?}", Customer::SAMPLE_1);
    }

    #[test]
    fn use_clone() {
        eprintln!("{:?}", Customer::SAMPLE_1.clone());

        let mut target = Customer::default();
        target.clone_from(&Customer {
            id: 2,
            name: "クマさん",
            address: "千葉県",
            ..Customer::SAMPLE_1.clone()
        });
        eprintln!("{:?}", target);
    }

    //    #[test]
    //    fn use_from() {
    //        let customer_data = vec!["10", "ガイさん", "", ""];
    //        let customer = Customer::from(&customer_data);
    //
    //        assert!(customer.id == 10);
    //    }
    #[test]
    fn use_try_from() {
        let customer_data = vec!["10", "ガイさん", "", ""];
        let customer = Customer::try_from(&customer_data);

        assert!(customer.unwrap().id == 10);
    }
}
