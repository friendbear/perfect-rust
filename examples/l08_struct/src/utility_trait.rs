use std::fmt::Display;

#[derive(Debug, Clone)]
struct Customer<'a> {
    id: u32,
    name: &'a str,
    address: &'a str,
    email: &'a str,
}
impl<'a> Customer<'a> {
    #[allow(dead_code)]
    pub const SAMPLE_1: Customer<'a> = Customer {
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
            name: "No name",
            address: "",
            email: "",
        }
    }
}
/* TODO
impl<'a> Into<(u32, &'a str, &'a str, &'a str)> for Customer<'a> {
    fn into(self, (id: u32, name: &'a str, address: &'a str, email: &'a str)) -> Self {
        Self {id name, address, email}
    }
}
*/

impl<'a> From<(u32, &'a str, &'a str, &'a str)> for Customer<'a> {
    fn from(val: (u32, &'a str, &'a str, &'a str)) -> Self {
        Self {
            id: val.0,
            name: val.1,
            address: val.2,
            email: val.3,
        }
    }
}
impl<'a> TryFrom<&Vec<&'a str>> for Customer<'a> {
    type Error = String;
    fn try_from(value: &Vec<&'a str>) -> Result<Self, Self::Error> {
        let id = value[0].parse::<u32>().map_err(|e| e.to_string())?;
        Ok(Customer {
            id,
            name: value.get(1).copied().ok_or("Missing name")?,
            address: value.get(2).copied().ok_or("Missing address")?,
            email: value.get(3).copied().ok_or("Missing email")?,
        })
    }
}

impl<'a> Display for Customer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}:{}:{}:{}",
            self.id, self.name, self.address, self.email
        )?;
        Ok(())
    }
}

mod test_utility_trait {

    #[test]
    fn use_debug() {
        eprintln!("{:?}", super::Customer::SAMPLE_1);
    }

    #[test]
    fn use_clone() {
        eprintln!("{:?}", super::Customer::SAMPLE_1.clone());

        let mut target = super::Customer::default();
        target.clone_from(&super::Customer {
            id: 2,
            name: "クマさん",
            address: "千葉県",
            ..super::Customer::SAMPLE_1.clone()
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
        let customer = super::Customer::try_from(&customer_data);

        assert!(customer.unwrap().id == 10);
    }

    #[test]
    fn use_format() {
        eprintln!("{}", super::Customer::SAMPLE_1);
    }
}
