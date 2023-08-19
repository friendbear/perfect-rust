use anyhow::Result;

pub trait Calculator {
    fn calc(&self) -> Result<u64> {
        todo!("Not implimented yet.")
    }
}

pub struct Rectangle {
    width: u64,
    height: u64
}

impl Calculator for Rectangle {
    fn calc(&self) -> Result<u64> {
        Ok(self.width * self.height)
    }
}

#[test]
fn use_rectangle() {
    let rect = Rectangle{ width: 100, height: 50};
    assert_eq!(rect.calc().unwrap(), 5000);
}