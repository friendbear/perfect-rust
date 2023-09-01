use lombok::*;

#[derive(Getter, GetterMut, Setter, NoArgsConstructor, AllArgsConstructor, ToString, Clone)]
pub struct ConnectParams {
    host: String,
    port: u16,
    dbname: String,
    user: String,
    password: String,
}
impl ConnectParams {
    pub fn connect_string(&self) -> String {
        format!(
            "host={} port={} dbname={} user={} password={}",
            self.host, self.port, self.dbname, self.user, self.password
        )
    }
}
