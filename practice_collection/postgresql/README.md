# PostgreSQL

## Docker

### run

```sh
docker run \
  --name pg \
  --detach \
  --publish 5432:5432 \
  --env POSTGRES_PASSWORD=admin \
  postgres
```

### exec

```sh
docker exec -it pg \
  psql -U postgres -d rust_sample
```

## 使用するテーブル構造

```mermaid
erDiagram
    product_category {
        int id
        string name
    }
    product ||--|{ product_category : contains
    product {
        int id
        string name
        int price
		int category_id
    }
```

## Repositoryパターン

```mermaid
classDiagram
class Repository~T,PK,UPD~ {
<<trait>>
}
class ProductRepository {
<<tuple struct>>
}
class Product {
<<struct>>
<<Entity>>
}
ProductRepository ..> Repository~T,PK,UPD~
ProductRepository ..> Product 
```

## QA

[[cargo clippy warning]arning: this `MutexGuard` is held across an `await` point](https://users.rust-lang.org/t/cargo-clippy-warning-arning-this-mutexguard-is-held-across-an-await-point/99225)

```rust
let config;
{
    let params = CONNECT_PARAMS.lock().unwrap();  // params lock here.
    config = params.connect_string().clone();
} // params drop here.
```