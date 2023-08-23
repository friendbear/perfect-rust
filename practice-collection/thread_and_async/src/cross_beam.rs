use crossbeam::thread;
#[derive(Default)]
struct Summery;

impl Summery {
    /// 合計値を求めるメソッド
    fn summery(&self, values: Vec<u64>) -> u64 {
        let mut total: u64 = 0;
        for value in values {
            total += value;
        }
        total
    }

    #[allow(dead_code)]
    /// グリーンスレッド
    /// 合計値を求めるスレッドを実行するメソッド
    /// thread::spawn -> ScopedJoinHandle<'_, u64>
    /// thread::scope -> Result<(), Box<dyn Any + Send>>
    fn summery_thread(&self) {
        thread::scope(|scope| {
            let handle1 = scope.spawn(|_| self.summery(vec![10, 20, 30, 40, 50]));
            let handle2 = scope.spawn(|_| self.summery(vec![100, 200, 300, 400, 500]));
            // スレッドの終了待ち
            let total1 = handle1.join().unwrap_or_else(|err| panic!("{:?}", err));
            let total2 = handle2.join().unwrap_or_else(|err| panic!("{:?}", err));

            println!("total1:{}, total2:{}", total1, total2);
        })
        .unwrap()
    }

    #[allow(dead_code)]
    /// ScopedThreadBuilder構造体の利用
    /// thread::scope -> Result<(), Box<dyn Any + Send>>
    fn use_builder(&self) {
        thread::scope(|scope| {
            let handle = scope
                .builder()
                .name(String::from("sum"))
                .stack_size(1024 * 3)
                .spawn(|_| self.summery(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]))
                .unwrap_or_else(|err| panic!("{:?}", err));
            let thread_result = handle.join().unwrap_or_else(|err| panic!("{:?}", err));
            println!("total={}", thread_result);
        })
        .unwrap()
    }
}

#[test]
fn thread_controller_1() {
    let summery = Summery::default();
    summery.summery_thread();
    summery.use_builder();
}
