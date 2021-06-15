use rand::Rng;
use std::time::Duration;
use tokio::time::sleep;

use crate::{data_processor::DataProcessor, network::Network};

pub struct Client<'a> {
    network: &'a Network,
    server: &'a DataProcessor,
}

impl<'a> Client<'a> {
    pub fn new(network: &'a Network, server: &'a DataProcessor) -> Self {
        Self { network, server }
    }

    pub async fn go(&self) {
        // Forever...
        loop {
            let id = rand::thread_rng().gen();
            isim_request__start!(|| id);
            self.network.traverse().await;
            self.server.request(id).await;
            self.network.traverse().await;
            isim_request__done!(|| id);
            sleep(Duration::from_secs(1)).await;
        }
    }
}
