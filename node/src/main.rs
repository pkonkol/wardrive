// TODO
// cross-compile it for radxa zero
// API Client
// tmp DB in Diesel(this)?
// WIFI scanning as separate thread/routine & module
// scanning & sending can restore from state

use client::{get_status, post_update};
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;


mod wifi;
mod client;
mod db;

mod prelude {
    pub use crate::client::*;
}


#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    // db::init(subscriber.clone());
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    info!("Hello, world!");
    info!("Got status '{}'", get_status().unwrap());
    info!("got response '{}'", post_update("Test123".to_string()).unwrap());
    db::establish_conn_sqlx().await.unwrap();
    db::init_sqlx().await.unwrap();
    db::test_sqlx().await.unwrap();
}
