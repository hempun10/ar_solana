use anchor_lang::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Data {
    key: String,
    value: String,
}

#[contract]
struct Contract {
    data: storage::HashMap<String, String>,
}

impl Contract {
    pub fn new() -> Self {
        Contract {
            data: storage::HashMap::new("data"),
        }
    }

    #[write]
    pub fn receive_data(&mut self) {
        // Send request to Postman API to retrieve data
        let client = Client::new();
        let res = client
            .get("https://api.openweathermap.org/?q=Pokhara&appid=55dbec5434ef18899504094e849892bc&units=metic")
            .send()
            .unwrap();
        let json_data: Data = res.json().unwrap();
        
        // Insert data into contract storage
        self.data.insert(json_data.key.clone(), json_data.value.clone());
    }

    #[write]
    pub fn send_data_to_express(&self, express_url: String) {
        // Convert data to JSON string
        let json_data = serde_json::to_string(&self.data.to_vec()).unwrap();

        // Send data to Express.js server
        let client = Client::new();
        client
            .post(&express_url)
            .json(&json_data)
            .send()
            .unwrap();
    }
}
fn main() {
    let mut contract = Contract::new();

    contract.receive_data();
    contract.send_data_to_express("http://localhost:3000/data".to_string());
}
