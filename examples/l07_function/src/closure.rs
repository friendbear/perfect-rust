fn closure_sum(values: &Vec<i32>) -> i32 {
    let sum = || {
        let mut sum = 0;
        for v in values.iter() {
            sum += v;
        }
        sum
    };
    sum()
}
fn closure_sum_move(values: &Vec<i32>) -> i32 {
    let sum = move || {
        let mut sum = 0;
        for v in values.iter() {
            sum += v;
        }
        sum
    };
    sum()
}

/// 初期値に対してスライス値を減算するclosure
fn return_closure_decrease_logic() -> impl FnOnce(i32, &[i32]) -> i32 {
    move |init, values| {
        let mut dec = init;
        for v in values {
            dec -= v;
        }
        dec
    }
}
fn use_closure<F>(init: i32, values: &[i32], f: F) -> i32
where
    F: FnOnce(i32, &[i32]) -> i32,
{
    let closure = f;
    closure(init, values)
}
#[cfg(test)]
mod closure {

    #[test]
    fn closure_sum() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = super::closure_sum(&v);
        assert_eq!(result, 45);
        eprintln!("{:?}", v);
    }
    #[test]
    fn closure_sum_move() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = super::closure_sum_move(&v);
        assert_eq!(result, 45);
        eprintln!("{:?}", v);

        let count = move || {
            let value = v.iter().fold(0, |acc, v| acc + v);
            value
        };
        eprintln!("count = {}", count());
        // Err use moved value v eprintln!("{:?}", v);
    }

    #[test]
    fn return_closure_decrease_logic() {
        let slice = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let init = 45;
        let closure = super::return_closure_decrease_logic();
        let result = closure(init, &slice);
        assert_eq!(result, 0);
        eprintln!("{:?}", slice);
    }

    #[test]
    fn use_closure() {
        let slice = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let init = 45;
        let result = super::use_closure(init, &slice, super::return_closure_decrease_logic());
        assert_eq!(result, 0);
    }
}
