use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

#[derive(Deserialize, Serialize, Debug)]
pub struct WifiEntry {
    pub id: i32,
    pub lat: f32,
    pub lon: f32,
    pub ssid: String,
}

#[derive(Deserialize, Serialize)]
pub struct OpenPorts {
    pub id: i32,
    pub ip: String,
    pub ports: Vec<i32>,
}

#[cfg(test)]
mod tests {
    use crate::tests_common::mock_entries;

    use super::*;

    #[test]
    fn json_demo() {
        let j = json!({
            "lat": 1.1,
            "lon": 2.2,
            "ssid": "serde1",
        });
        println!("j debug: {:?}", j);
        println!("j: {}", j);
        println!("j to s: {}", j.to_string());
    }

    #[test]
    fn deserialize_single_entry() -> anyhow::Result<()>{
        let s = r#"
        {
            "lat": 1.1,
            "lon": 2.2,
            "ssid": "serde1"
        }"#;
        let e: WifiEntry = serde_json::from_str(s)?;
        println!("e debug: {:?}", e);
        Ok(())
    }


    #[test]
    fn serialize_single_entry() -> anyhow::Result<()>{
        let es = mock_entries();
        let e = es.get(0).unwrap();
        let j = serde_json::to_string(e)?;
        println!("j: {}", j);
        Ok(())
    }

    #[test]
    fn serialize_vec_entries() -> anyhow::Result<()>{
        let e = mock_entries();
        let j = serde_json::to_string(&e)?;
        println!("j: {}", j);
        Ok(())
    }

}