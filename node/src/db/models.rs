use serde::{Deserialize, Serialize};
//use serde_json::{Map, Value};

#[derive(Deserialize, Serialize)]
pub struct WifiEntry {
    pub id: i32,
    pub lat: String,
    pub lon: String,
    pub ssid: bool,
}

#[derive(Deserialize, Serialize)]
pub struct OpenPorts {
    pub id: i32,
    pub ip: String,
    pub ports: Vec<i32>,
}