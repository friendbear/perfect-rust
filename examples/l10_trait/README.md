## Topic
- トレイトの基本
- ジェネリクストレイト
```rust
trait トレイト名<T[,..]> {
     fn メソッドまたは関連型関数名([引数名: T,..]) [-> T] {
         処理
     } 
}
```  
- 幽霊フィールド(PhantomData<T>)
- 関連型トレイト(Association Type)

```rust
trait トレイト名 {
    type 関連型;
     fn メソッドまたは関連型関数名([引数名: Self::関連型,..]) [-> Self::関連型] {
         処理
     } 
}
```

## ReadService

Box型とdynamicで抽象化

```mermaid
classDiagram
class ReadService {
<<struct>>
-csv_reader: Box~dyn CsvReader~T~~
-json_reader: Box~dyn JsonReader~T~~

+new(): Self
+csv_read(file_path: &str): Result~Vec~T~~
+json_read(file_path: &str): Result~Vec~T~~
}

class CsvReader~T~ {
<<trait>>
+read(file_path: &str): Result~T~
}

class JsonReader~T~ {
<<trait>>
+read(file_path: &str): Result~T~
}
ReadService "1" o-- "1" CsvReader 
ReadService "1" o-- "1" JsonReader 
```
