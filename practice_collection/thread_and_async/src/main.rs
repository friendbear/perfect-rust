mod messaging_crossbeam;
mod messaging_std;
fn main() {
    messaging_std::execute();
    messaging_crossbeam::execute();
}
