use serde::Deserialize;
use serde_json::from_str;
use crossbeam::channel::bounded;
use crossbeam::thread;
use crossbeam::channel::{Receiver, Sender};
#[derive(Deserialize, Clone)]
pub struct Station {
    id: u32,
    name: String,
}
#[derive(Clone)]
pub struct Stations(Vec<Station>);
impl Stations {
    pub fn from_json(json: &str) -> Self {
        Self (from_str::<Vec<Station>>(json).expect("JSON Parse error."))
    }
    pub fn search_by_name(&self, name: &str) -> Result<Station, String> {
        self.0
            .clone()
            .into_iter()
            .find(|station| station.name.eq(name))
            .ok_or("Not found.".to_owned())
    }
}

#[allow(dead_code)]
pub struct Client;
impl Client {
    /// Providerに検索要求を出し、結果を受け取るメソッド
    /// ### 引数: Providerへの送信チャネル p_sender: Sender<String>
    /// ### 引数: Clientの受信チャネル c_receiver: Receiver<String>
    #[allow(dead_code)]
    pub fn search_request(p_sender: Sender<String>, c_receiver: Receiver<String>) {
        let mut station_name =
            ["錦糸町".to_owned(), "さいたま".to_owned(), "end".to_owned(), "千葉".to_owned()].into_iter();
        loop {
            let entry_name = station_name.next().unwrap();
            p_sender
                .send(entry_name.clone())
                .unwrap_or_else(|err| println!("{:?}", err));
            if entry_name == "end" {
                break;
            }
            // Providerっからの検索結果受信
            c_receiver
                .recv()
                .map(|result| {
                    println!("recive:{:?}", result);
                })
                .unwrap_or_else(|err| println!("{:?}", err));
        }
        println!("Client 終了");
    }
}
#[allow(dead_code)]
pub struct Provider;
impl Provider {
    /// 受信した駅名で検索した結果を送信するメソッド
    #[allow(dead_code)]
    pub fn search_service(c_sender: Sender<String>, p_receiver: Receiver<String>) {
        let stations = Stations::from_json(
            r#"[
                {
                    "id": 1,
                    "name": "千葉"
                },
                {
                    "id": 2,
                    "name": "錦糸町"
                }
            ]
            "#,
        );
        loop {
            let entry_name = p_receiver.recv().unwrap_or_else(|err| panic!("{:?}", err));
            if entry_name == "end" {
                break;
            }

            let result = match stations.search_by_name(&entry_name) {
                Ok(station) => format!("{}", station.id),
                Err(e) => e,
            };
            c_sender
                .send(result)
                .unwrap_or_else(|err| panic!("{}", err))
        }
        println!("Provider 終了")
    }
}

#[allow(dead_code)]
pub fn execute() {
    let (c_sender, c_receiver) = bounded::<String>(5);
    let (p_sender, p_receiver) = bounded::<String>(5);

    thread::scope(|scope| {
        let p_handle = scope.spawn(|_| {
            Provider::search_service(c_sender, p_receiver);
        });
        let c_handle = scope.spawn(|_| {
            Client::search_request(p_sender, c_receiver);
        });
        // 終了待ち
        p_handle.join().unwrap_or_else(|err| panic!("{:?}", err));
        c_handle.join().unwrap_or_else(|err| panic!("{:?}", err));
    }).unwrap();
}
