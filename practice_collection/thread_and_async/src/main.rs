use async_std::task;

mod exclusive_mutex;
mod exclusive_rwlock;
mod exclusive_shradelock;
mod messaging_crossbeam;
mod messaging_std;
use thread_and_async::task::task_controller::customer_controller;

fn main() {
    messaging_std::execute();
    messaging_crossbeam::execute();
    exclusive_mutex::calc_avg_and_sum_use_mutex();
    exclusive_rwlock::Calculator::calc_sum_and_avg_use_rwlock()
        .map(|(sum, avg)| println!("avg:{}, sum:{}", avg, sum))
        .unwrap();
    exclusive_shradelock::Calculator
        .calc_sum_and_avg_use_sharded_lock()
        .map(|(sum, avg)| println!("avg:{}, sum:{}", avg, sum))
        .unwrap();

    task::block_on(async {
        customer_controller().await;
    });
}
