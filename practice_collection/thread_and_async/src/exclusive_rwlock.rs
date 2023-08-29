use anyhow::Result;
use core::panic;
use std::ops::Div;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, RwLock};
use std::thread::Builder;

#[derive(Debug)]
pub struct Calculator;
impl Calculator {
    /// 計算データを生成する
    #[allow(dead_code)]
    fn create_data(
        sender: (Sender<()>, Sender<()>),
        params: Arc<RwLock<Vec<u64>>>,
    ) -> Result<String> {
        let mut vals = params.write().unwrap(); // write lock
        for num in 1..=100 {
            vals.push(num * 100);
        }
        // 生成完了を通知
        sender.0.send(()).unwrap_or_else(|err| panic!("{:?}", err));
        sender.1.send(()).unwrap_or_else(|err| panic!("{:?}", err));
        Ok(String::from("計算値の生成終了"))
    }

    // 合計値を求める関数
    #[allow(dead_code)]
    fn calc_sum(reciver: Receiver<()>, params: Arc<RwLock<Vec<u64>>>) -> Result<String> {
        reciver.recv().unwrap_or_else(|err| panic!("{:?}", err));
        let vals = params.read().unwrap(); // read lock
        let mut total = 0;
        for value in vals.iter() {
            total += value;
        }
        Ok(total.to_string())
    }

    // 平均値を求める関数
    #[allow(dead_code)]
    fn calc_avg(reciver: Receiver<()>, params: Arc<RwLock<Vec<u64>>>) -> Result<String> {
        reciver.recv().unwrap_or_else(|err| panic!("{:?}", err));
        let vals = params.read().unwrap();
        let mut total = 0;
        for value in vals.iter() {
            total += value;
        }
        Ok(total.div(vals.iter().count() as u64).to_string())
    }

    // スレッドを実行する
    #[allow(dead_code)]
    pub fn calc_sum_and_avg_use_rwlock() -> Result<(String, String)> {
        let (s_sender, s_reciver) = mpsc::channel::<()>();
        let (a_sender, a_reciver) = mpsc::channel::<()>();
        let params: Arc<RwLock<Vec<u64>>> = Arc::new(RwLock::new(Vec::<u64>::new()));

        let handle_create = Builder::new()
            .name("create".to_owned())
            .stack_size(1024 * 3);
        let params_1 = Arc::clone(&params);
        handle_create.spawn(move || Self::create_data((s_sender, a_sender), params_1).unwrap())?;

        let builder = Builder::new().name("sum".to_owned()).stack_size(1024 * 3);
        let params_1 = Arc::clone(&params);
        let join_handle_sum =
            builder.spawn(move || Self::calc_sum(s_reciver, params_1).unwrap())?;

        let builder = Builder::new().name("avg".to_owned()).stack_size(1024 * 3);
        let params_1 = Arc::clone(&params);
        let join_handle_avg =
            builder.spawn(move || Self::calc_avg(a_reciver, params_1).unwrap())?;
        Ok((
            join_handle_sum.join().unwrap(),
            join_handle_avg.join().unwrap(),
        ))
    }
}
