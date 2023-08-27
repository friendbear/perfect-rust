use anyhow::Result;
use core::panic;
use crossbeam::channel::{bounded, Receiver, Sender};
use crossbeam::sync::ShardedLock;
use crossbeam::thread;
use std::ops::Div;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct Calculator;
impl Calculator {
    /// 計算データを生成する
    #[allow(dead_code)]
    fn create_data(
        &self,
        sender: (Sender<()>, Sender<()>),
        params: Arc<ShardedLock<Vec<u64>>>,
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
    fn calc_sum(
        &self,
        reciver: Receiver<()>,
        params: Arc<ShardedLock<Vec<u64>>>,
    ) -> Result<String> {
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
    fn calc_avg(
        &self,
        reciver: Receiver<()>,
        params: Arc<ShardedLock<Vec<u64>>>,
    ) -> Result<String> {
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
    pub fn calc_sum_and_avg_use_sharded_lock(&self) -> anyhow::Result<(String, String)> {
        let r = thread::scope(|scope| {
            let (s_sender, s_reciver) = bounded::<()>(5);
            let (a_sender, a_reciver) = bounded::<()>(5);
            let params: Arc<ShardedLock<Vec<u64>>> = Arc::new(ShardedLock::new(Vec::<u64>::new()));

            let params_1 = Arc::clone(&params);
            let _handle_create = scope
                .builder()
                .name("create".to_owned())
                .stack_size(1024 * 3)
                .spawn(|_| self.create_data((s_sender, a_sender), params_1))
                .unwrap_or_else(|err| panic!("{:?}", err));

            let params_2 = Arc::clone(&params);
            let handle_sum = scope
                .builder()
                .name("sum".to_owned())
                .stack_size(1024 * 3)
                .spawn(|_| self.calc_sum(s_reciver, params_2))
                .unwrap_or_else(|err| panic!("{:?}", err));

            let params_3 = Arc::clone(&params);
            let handle_avg = scope
                .builder()
                .name("avg".to_owned())
                .stack_size(1024 * 3)
                .spawn(|_| self.calc_avg(a_reciver, params_3))
                .unwrap_or_else(|err| panic!("{:?}", err));

            Ok((
                handle_sum.join().unwrap().unwrap(),
                handle_avg.join().unwrap().unwrap(),
            ))
            //println!("{}/{}", handle_sum.join().unwrap().unwrap(), handle_avg.join().unwrap().unwrap())
        });
        r.unwrap()
    }
}
