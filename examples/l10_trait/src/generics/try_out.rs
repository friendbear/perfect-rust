use crate::generics::entities::Product;
use crate::generics::traits::{CsvReader, JsonReader};
use crate::generics::traits_impl::{CsvReaderImpl, JsonReaderImpl};

use super::service::ReadService;

#[allow(dead_code)]
pub fn use_generics_method() {
    // ファイルパスを生成する
    let file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/number.csv");
    println!("{}", file_path);
    let csv_reader = CsvReaderImpl::<(u32, String)>::default();
    let deserialize_csv = csv_reader.read(file_path).unwrap();
    println!("{:?}", deserialize_csv);

    let file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/products.json");
    println!("{}", file_path);
    let json_reader = JsonReaderImpl::<Product>::default();
    let deserialize_json = json_reader.read(file_path).unwrap();
    println!("{:?}", deserialize_json);
}

#[allow(dead_code)]
pub fn use_service_metthod() {
    let read_service = ReadService::<Product>::new();
    let file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/products.csv");
    let product_from_csv = read_service.csv_reader(file_path).unwrap();
    println!("{:?}", product_from_csv);

    let file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/products.json");
    let product_from_json = read_service.json_reader(file_path).unwrap();
    println!("{:?}", product_from_json);
}
