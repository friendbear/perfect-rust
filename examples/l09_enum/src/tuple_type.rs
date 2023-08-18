#[repr(u64)] // u8, u16, u32, u64, c
#[derive(Debug)]
enum Season<'a> {
    Spring(u8, Vec<&'a str>),
    Summer(u8, Vec<&'a str>),
    Autumn(u8, Vec<&'a str>),
    Winter(u8, Vec<&'a str>),
}

impl<'a> Season<'a> {
    pub fn format_valiant(&self) -> String {
        match self {
            Self::Spring(x, y) => format!("春:{}ヶ月 {:?}", x, y),
            Self::Summer(x, y) => format!("春:{}ヶ月 {:?}", x, y),
            Self::Autumn(x, y) => format!("春:{}ヶ月 {:?}", x, y),
            Self::Winter(x, y) => format!("春:{}ヶ月 {:?}", x, y),
        }
    }
}

#[test]
fn use_tuple() {
    let spring = Season::Spring(3, vec!["3月", "4月", "5月"]);
    let spring = spring.format_valiant();
    println!("{}", spring);
    println!("{:?}", spring);
}
