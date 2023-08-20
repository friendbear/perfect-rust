
use crate::assosiation::entities::Product;
use crate::assosiation::traits::{CsvReader, JsonReader};
use crate::assosiation::traits_impl::{CsvReaderImpl, JsonReaderImpl};

pub fn use_association_method() {
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