use async_std::task::*;
use num_traits::{NumOps, Num};

async fn calc_sum<T>(values: Vec<T>) -> T 
where 
    T: Num + NumOps + Copy,
{
    let mut total: T = T::zero(); // 初期値をゼロに設定
    for value in values.iter() {
        total = total + *value;
    }
    total
}
pub async fn use_builder() {
    let task1 = Builder::new().name(String::from("task1")).spawn(async {
            let sum =calc_sum::<u32>([10, 20, 30, 40, 50].to_vec()).await;
            sum
        });
    let task2 = Builder::new().name(String::from("task2")).spawn(async {
            let sum = calc_sum::<u32>([100, 200, 300, 400, 500].to_vec()).await;
            sum
        });
    match task1 {
        Ok(result) => println!("{}", result.await),
        Err(error) => panic!("{:?}", error),
    };
    match task2 {
        Ok(result) => println!("{}", result.await),
        Err(error) => panic!("{:?}", error),
    };
}
#[async_std::main]
async fn main() {
    let total = calc_sum::<f32>([10.5, 76.2, 99.2].to_vec()).await;
    println!("total:{}", total);
    use_builder().await;
}
