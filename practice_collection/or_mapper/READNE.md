# O/R Mapper

## SeaORM

```sh
cargo install sea-orm-cli
```

### How to use

default database url .env

sea-orm-cli [-h|-v|-V]

sub commands

* generate
* help
* migrate

```sh
sea-orm-cli generate entity -u postgres://usr:pass@host:5432/dbname -o src/modules \
    --with-serde (none, serialize, deserialize, both) [default: none]
```
