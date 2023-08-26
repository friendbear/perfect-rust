mod messaging_std;
mod messaging_crossbeam;
fn main() {
    messaging_std::execute();
    messaging_crossbeam::execute();
}
