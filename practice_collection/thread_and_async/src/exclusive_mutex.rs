use anyhow::Result;
use std::ops::Div;
use std::sync::{Arc, Mutex};
use std::thread::Builder;
use std::thread::{self, JoinHandle};

#[derive(Debug)]
pub struct Calculator;
impl Calculator {
    /// Mutex lock
    /// param Arc<Mutex<Vec<u64>>>
    #[allow(dead_code)]
    fn calc_sum(param: Arc<Mutex<Vec<u64>>>) -> u64 {
        let mut total: u64 = 0;

        let values = param.lock().unwrap_or_else(|err| panic!("{:?}", err));

        for value in values.iter() {
            total += value;
            println!("{}/{}", thread::current().name().unwrap(), total);
        }
        total
    }

    #[allow(dead_code)]
    fn calc_avg(param: Arc<Mutex<Vec<u64>>>) -> u64 {
        let mut sum: u64 = 0;
        let values = param.lock().unwrap_or_else(|err| panic!("{:?}", err));
        let count: u64 = values.iter().count() as u64;
        for value in values.iter() {
            sum += value;
        }
        sum.div(count)
    }

    #[allow(dead_code)]
    pub fn calc_avg_and_sum_use_mutex(values: Vec<u64>) -> Result<(u64, u64)> {
        let params = Arc::new(Mutex::<Vec<u64>>::new(values));
        let mut handles: Vec<JoinHandle<u64>> = Vec::with_capacity(2);

        let builder = Builder::new().name("avg".to_owned()).stack_size(1024 * 3);
        let clone_param = Arc::clone(&params);
        handles.push(builder.spawn(move || Self::calc_avg(clone_param))?);

        let builder = Builder::new().name("sum".to_owned()).stack_size(1024 * 3);
        let clone_param = Arc::clone(&params);
        handles.push(builder.spawn(move || Self::calc_sum(clone_param))?);
        // The handles pop in reverse order, so sum is first
        let sum = handles.pop().unwrap().join().unwrap();
        let avg = handles.pop().unwrap().join().unwrap();
        Ok((avg, sum))
    }
}

#[allow(dead_code)]
pub fn calc_avg_and_sum_use_mutex() {
    let params = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let (avg, sum) = Calculator::calc_avg_and_sum_use_mutex(params).unwrap();
    println!("avg:{}, sum:{}", avg, sum);
}
