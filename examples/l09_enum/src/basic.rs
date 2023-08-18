use std::fmt::Display;
use std::fmt::Formatter;

#[repr(u64)] // u8, u16, u32, u64, c
#[derive(Debug)]
enum Season {
    Spring = 1,
    Summer,
    Autumn,
    Winter
}
impl Display for Season {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Spring => write!(f, "Spring(春):{}", Self::Spring as u8),
            Self::Summer => write!(f, "Summer(夏):{}", Self::Summer as u8),
            Self::Autumn => write!(f, "Autumn(秋):{}", Self::Autumn as u8),
            Self::Winter => write!(f, "Winter(冬):{}", Self::Winter as u8)
        }?;
        Ok(())
    }
}

#[cfg(test)]
mod test_basic {
    use super::*;

    #[test]
    fn use_season() {
        let summer = Season::Summer;
        let winter = Season::Winter;
        let summer_num = summer as u8;
        let winter_num = winter as u8;
        assert!(summer_num == 2);
        assert!(winter_num == 4);
    }

    #[test]
    fn usee_fmt() {
        println!("{}", Season::Summer);
        println!("{}", Season::Winter);
    }
}