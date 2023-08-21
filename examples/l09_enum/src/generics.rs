#[repr(u64)] // u8, u16, u32, u64, c
#[derive(Debug)]
#[allow(dead_code)]
enum Season<T> {
    Spring(u8, T),
    Summer(u8, T),
    Autumn(u8, T),
    Winter(u8, T),
}

impl<T> Season<T>
where
    T: std::iter::IntoIterator,
{
    #[allow(dead_code)]
    pub fn get_month(&self) -> &T {
        match self {
            Self::Spring(_, month) => month,
            Self::Summer(_, month) => month,
            Self::Autumn(_, month) => month,
            Self::Winter(_, month) => month,
        }
    }
}

#[test]
fn use_generics() {
    use std::collections::LinkedList;
    let spring = Season::Spring::<Vec<&str>>(3, vec!["3月", "4月", "5月"]);
    let spring = spring.get_month();

    assert_eq!(spring, &["3月", "4月", "5月"]);
    //    let summer = Season::Summer::<Box<[&str]>>(3, Box::new(["6月", "7月", "8月"]));
    //    let summer = summer.get_month(); error => Box is not impliment IntoIterator

    let winter = Season::Winter::<LinkedList<&str>>(3, LinkedList::from(["12月", "1月", "2月"]));
    let _winter = winter.get_month();
}
