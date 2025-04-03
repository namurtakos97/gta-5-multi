use reqwest::blocking::Client;
use serde_json::Value;

pub struct NetworkClient {
    client: Client,
}

impl NetworkClient {
    pub fn new() -> Self {
        NetworkClient {
            client: Client::new(),
        }
    }

    pub fn send_data(&self, url: &str, data: &Value) {
        let _response = self.client.post(url).json(data).send();
    }

    pub fn fetch_data(&self, url: &str) -> Value {
        let response = self.client.get(url).send().unwrap();
        response.json().unwrap()
    }
}