mod messaging_crossbeam;
mod messaging_std;
mod exclusive_mutex;
fn main() {
    messaging_std::execute();
    messaging_crossbeam::execute();
    exclusive_mutex::calc_avg_and_sum_use_mutex();
}
