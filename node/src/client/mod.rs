mod request;

use super::db;
pub use request::{get_status, post_update, send_entries};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use tokio::time::{sleep, Duration};

const SEND_THRES: usize = 10;
static MASTER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);


pub async fn status() -> String {
    "DB status".to_string()
}

/// Periodically check if there's enough records in the DB and if so try to
/// publish them to the master.
pub async fn update_loop() {
    loop {
        // TODO skip on error instead of unwrap
        let n = db::entries_available().await.unwrap();
        if n > SEND_THRES {
            let e = db::get_entries().await.unwrap();
            let r = send_entries(&e).await.unwrap();
            // if r.receive sucess
            // remove by range? by id's? TODO
            db::remove_entries(e);
        } 
        sleep(Duration::from_secs(30));
    }
}