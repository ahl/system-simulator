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
            // Generate a random id simply to correlate start and done probes.
            let id = rand::thread_rng().gen();
            isim_request__start!(|| id);
            // Send the request over the network.
            self.network.traverse().await;
            // Make a request to the server.
            self.server.request(id).await;
            // Return the response over the network.
            self.network.traverse().await;
            isim_request__done!(|| id);

            // Pause for a second.
            sleep(Duration::from_secs(1)).await;
        }
    }
}
