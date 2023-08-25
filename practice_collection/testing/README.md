# テスト

## testsディレクトリ

- test_module.rs を配置
- testから始まる関数が必要

> Tips:
>  
> testsディレクトリからsrcディレクトリのテストターゲットをモジュールとして読み込む

```rust
mod my_module {
    include!("../../src/target.rs"); // テスト対象のソース
}
#[cfg(test)]
mod simple_test {
    use super::my_module::{Guest,SampleError};
}
```
