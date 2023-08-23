use std::thread;
use std::thread::{Builder, JoinHandle};
use std::time::Duration;

#[allow(dead_code)]
/// シンプルなスレッド
/// JoinHandle<u64> を返す
fn summery_thread_1(name: String, values: Vec<u64>) -> JoinHandle<u64> {
    thread::spawn(move || {
        let mut total: u64 = 0;
        for value in values {
            total += value;
            thread::sleep(Duration::from_millis(2)); // sleep 2 min
            println!("{}: total={}", name, total);
        }
        total
    })
}

#[allow(dead_code)]
/// Builder構造体
/// - name()
/// - stack_size()
/// - spawn() ->Result<JoinHandle<T>>
fn summery_thread_2(name: String, values: Vec<u64>) -> std::thread::Result<JoinHandle<u64>> {
    let builder = Builder::new().name(name).stack_size(1024 * 3);
    let join_handle = builder.spawn(|| {
        let mut total: u64 = 0;
        for value in values {
            total += value;
            thread::sleep(Duration::from_millis(100));
            println!(
                "{}: total={}",
                std::thread::current().name().unwrap(),
                total
            );
        }
        total
    });
    Ok(join_handle.unwrap())
}

#[test]
fn thread_controller_1() {
    let thread1 = summery_thread_1(String::from("t#1"), vec![10, 20, 30, 40, 50]);
    let thread2 = summery_thread_1(String::from("t#2"), vec![100, 200, 300, 400, 500]);

    let t1 = thread1.join().map_err(|err| panic!("{:?}", err)).unwrap();
    let t2 = thread2.join().map_err(|err| panic!("{:?}", err)).unwrap();
    assert_eq!(t1, 150);
    assert_eq!(t2, 1500);
}

#[test]
fn thread_controller_2() {
    let thread1 = summery_thread_2(String::from("t#1"), vec![10, 20, 30, 40, 50]);
    let thread2 = summery_thread_2(String::from("t#2"), vec![100, 200, 300, 400, 500]);

    let t1 = thread1
        .unwrap()
        .join()
        .map_err(|err| panic!("{:?}", err))
        .unwrap();
    let t2 = thread2
        .unwrap()
        .join()
        .map_err(|err| panic!("{:?}", err))
        .unwrap();
    assert_eq!(t1, 150);
    assert_eq!(t2, 1500);
}
