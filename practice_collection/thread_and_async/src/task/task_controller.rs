use crate::task::client::Client;
use crate::task::customer::Customer;
use crate::task::writer::SampleWriter;
use async_std::channel::unbounded;
use async_std::task;

pub async fn customer_controller() {
    let (csv_sender, csv_receiver) = unbounded::<Customer>();
    let (json_sender, json_receiver) = unbounded::<Customer>();

    let csv_join_handle = task::spawn(async move {
        let writer = SampleWriter::<Customer>::default();

        writer.csv_writer(csv_receiver).await
    });
    let json_join_handle = task::spawn(async move {
        let writer = SampleWriter::<Customer>::default();

        writer.json_writer(json_receiver).await
    });
    let client_join_handle = task::spawn(async move {
        let client = Client;

        client.entry((csv_sender, json_sender)).await
    });

    println!("{:?}", csv_join_handle.await.unwrap());
    println!("{}", json_join_handle.await.unwrap());
    println!("{}", client_join_handle.await.unwrap());
}
