#[cfg(test)]
mod test_panic {
    use std::env::current_dir;
    use std::fs::File;
    #[test]
    #[should_panic]
    fn use_expect() {
        let file_path = current_dir()
            .map(|path_buf| path_buf.join("notfound.txt"))
            .map_err(|err| panic!("{}", err))
            .unwrap();
        let _file = File::open(file_path).expect("ファイルが存在しないので処理を中断します");
    }

    #[test]
    #[should_panic]
    fn use_panic() {
        let file_path = current_dir()
            .map(|path_buf| path_buf.join("notfound.txt"))
            .map_err(|err| panic!("{}", err))
            .unwrap();
        let file = File::open(file_path);
        if file.is_err() {
            panic!(
                "{},{:?}",
                "ファイルが存在しないので処理を終了します",
                file.err().unwrap()
            );
        }
    }
}
