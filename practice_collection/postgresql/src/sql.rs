use anyhow::{Error, Result};
use once_cell::sync::Lazy;
use std::env;
use std::sync::Mutex;
use yaml_rust::{Yaml, YamlLoader};

static SQLS: Lazy<Mutex<Yaml>> =
    Lazy::new(|| init_sqls().unwrap_or_else(|err| panic!("{:?}", err)));

fn init_sqls() -> Result<Mutex<Yaml>> {
    let current = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/sql.yaml");
    dbg!(&current);
    let str_yaml = std::fs::read_to_string(current)?;
    let values: Vec<Yaml> = YamlLoader::load_from_str(str_yaml.as_str())?;
    dbg!(&values);
    let result = Mutex::new(values[0].clone());
    Ok(result)
}

/// return sql
pub async fn get_sql(table_name: &str, method_name: &str) -> Result<String> {
    dbg!("------");
    let yaml = SQLS.lock().unwrap();
    let sqls = match yaml[table_name].as_hash() {
        Some(sqls) => sqls,
        None => return Err(Error::msg("Not found table")),
    };
    let sql = match sqls.get(&Yaml::String(method_name.to_owned())) {
        Some(sql) => sql.as_str().unwrap(),
        None => return Err(Error::msg("Not found method")),
    };
    Ok(sql.to_owned())
}
