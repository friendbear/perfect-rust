use crossbeam::sync::WaitGroup;
use crossbeam::thread;
use std::sync::Arc;
use std::sync::Barrier;
use std::thread::Builder;
#[derive(Default)]
struct Summery;

impl Summery {
    /// 合計値を求めるメソッド
    fn summery(&self, values: Vec<u64>) -> u64 {
        Self::summery_a(values)
    }
    /// 合計値を求める関連型関数
    fn summery_a(values: Vec<u64>) -> u64 {
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

    /// スレッド終了の同期化
    /// Barrier構造体を利用した終了の同期化
    /// threadの終了を完了を待って結果を取り出す
    #[allow(dead_code)]
    fn use_barrier() {
        // スレッドのハンドルを格納するVec
        let mut join_handles = Vec::with_capacity(3);
        // ArcでラップしたBarrierを生成する
        let barrier = Arc::new(Barrier::new(3));
        let mut num: u64 = 0;
        while num <= 2 {
            let arc = Arc::clone(&barrier);
            join_handles.push(
                // threadをVecに登録する
                // return Result<JoinHandle<u64, Error>
                Builder::new()
                    .name(format!("{}{}", "summary", num))
                    .stack_size(1024 * 5)
                    .spawn(move || {
                        let data: Vec<u64> = vec![10 + num, 20 + num, 30 + num, 40 + num, 50 + num];
                        let result = Self::summery_a(data);
                        // 🦀wait()メソッドで終了を待つ
                        let wresult = arc.wait();
                        eprintln!(
                            "{} finished, is_leader:{}",
                            std::thread::current().name().unwrap(),
                            wresult.is_leader()
                        );
                        result
                    })
                    .unwrap_or_else(|err| panic!("{:?}", err)),
            );
            num += 1;
        }
        for join_handle in join_handles {
            // thread nameを取り出す 所有権が移動するのでcloneする
            let thread = join_handle.thread().clone();
            let result = join_handle.join().unwrap_or_else(|err| panic!("{:?}", err));
            println!("thread name:{}, result:{}", thread.name().unwrap(), result);
        }
    }

    #[allow(dead_code)]
    /// crossbeam::sync::WaitGroup
    /// 
    fn use_wait_group(&self) {
        thread::scope(|scope| {
            let mut join_handles = Vec::with_capacity(3);
            let wait_group = WaitGroup::new();
            let mut num: u64 = 0;
            while num <= 2 {
                let wg = wait_group.clone();
                join_handles.push(
                    scope.builder()
                    .name(format!("{}{}", "summary", num))
                    .stack_size(1024 * 3)
                    .spawn(move |_| {
                            let result = self.summery(vec![
                                10 * num,
                                20 * num,
                                30 * num,
                                40 * num,
                                50 * num,
                            ]);
                            drop(wg);
                        result
                    })
                        .unwrap_or_else(|err| panic!("{:?}", err)),
                );
                num += 1;
            }
            wait_group.wait();
            for join_handle in join_handles {
                // thread nameを取り出す 所有権が移動するのでcloneする
                let thread = join_handle.thread().clone();
                let result = join_handle.join().unwrap_or_else(|err| panic!("{:?}", err));
                println!("thread name:{}, result:{}", thread.name().unwrap(), result);
            }
        })
        .unwrap();
    }
}

#[test]
fn thread_controller_1() {
    let summery = Summery::default();
    summery.summery_thread();
    summery.use_builder();
    Summery::use_barrier(); // threadの動機を取る
    /*
    summary2 finished, is_leader:true
    summary0 finished, is_leader:false
    summary1 finished, is_leader:false
    thread name:summary0, result:150
    thread name:summary1, result:155
    thread name:summary2, result:160
     */
}
