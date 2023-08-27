mod messaging_crossbeam;
mod messaging_std;
mod exclusive_mutex;
mod exclusive_rwlock;
fn main() {
    messaging_std::execute();
    messaging_crossbeam::execute();
    exclusive_mutex::calc_avg_and_sum_use_mutex();
    exclusive_rwlock::Calculator::calc_sum_and_avg_use_rwlock().map(|(sum, avg)| println!("avg:{}, sum:{}", avg, sum)).unwrap();
}
