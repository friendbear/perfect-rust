// async_std::task::block_on()
fn main() {
    async_std::task::block_on(async {
        let values = [10, 20, 30, 40, 50];
        let total = calc_sum(values.to_vec()).await;
        println!("{:?} sum is {}", values, total);
    });
}

async fn calc_sum(values: Vec<u64>) -> u64 {
    let mut total: u64 = 0;
    for value in values {
        total += value;
        std::thread::sleep(std::time::Duration::from_secs(2));
        println!("total:{}", total);
    }
    total
}
