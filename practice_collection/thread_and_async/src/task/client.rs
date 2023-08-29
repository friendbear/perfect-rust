use crate::task::customer::Customer;
use anyhow::{Context, Result};
use async_std::channel::Sender;

pub struct Client;
impl Client {
    pub async fn entry(&self, sender: (Sender<Customer>, Sender<Customer>)) -> Result<String> {
        let customer = Customer::new("はしちゃん".to_owned(), "hashichan@example.com".to_owned());
        let _ = sender.0.send(customer.clone()).await.context("context");
        let _ = sender.1.send(customer.clone()).await.context("context");
        Ok(String::from("Client Finished"))
    }
}
