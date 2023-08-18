/// 座標を表すタプル型構造体
struct Coordinate(usize, usize);

/// ユニット型構造体の生成
struct OneState;

#[cfg(test)]
mod test_tuple_unit_type {
    use super::{Coordinate, OneState};
    use std::borrow::Borrow;
    #[test]
    fn generate_tuple() {
        let coordinate = Coordinate(100, 200);
        assert!(coordinate.0 == 100 && coordinate.1 == 200);
    }

    #[test]
    fn generate_unit() {
        let one_state = OneState;
        let two_state = OneState;

        let one = one_state.borrow();
        let two = two_state.borrow();
        eprintln!("unit one addr = {:p}", one);
        eprintln!("unit two addr = {:p}", two);
    }
}
