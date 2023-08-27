use anyhow::Result;
use std::ops::Div;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::Builder;
use std::time::Duration;
#[derive(Debug)]
pub struct Calculator;
impl Calculator {
    /// Mutex lock
    /// param Arc<Mutex<Vec<u64>>>
    pub fn calc_sum(param: Arc<Mutex<Vec<u64>>>) -> u64 {
        let mut total: u64 = 0;

        let values = param.lock().unwrap_or_else(|err| panic!("{:?}", err));

        for value in values.iter() {
            total += value;
            println!("{}/{}", thread::current().name().unwrap(), total);
        }
        total
    }

    pub fn calc_avg(param: Arc<Mutex<Vec<u64>>>) -> u64 {
        let mut sum: u64 = 0;
        let values = param.lock().unwrap_or_else(|err| panic!("{:?}", err));
        let count: u64 = values.iter().count() as u64;
        for value in values.iter() {
            sum += value;
        }
        sum.div(count)
    }
}
