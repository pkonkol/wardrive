#[cfg(test)]

// use crate::db::WifiEntry;
use super::db::WifiEntry;

pub fn mock_entries() -> Vec<WifiEntry> {
    vec![
        WifiEntry { id: 1, lat: 18.5, lon: 50.2, ssid: "wifi1".to_string() },
        WifiEntry { id: 2, lat: 11.8, lon: 20.1, ssid: "wifi2".to_string() },
        WifiEntry { id: 3, lat: 8.3, lon: 30.0, ssid: "wifi3".to_string() },
    ]
}