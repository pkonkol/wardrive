// TODO
// cross-compile it for radxa zero
// API Client
// tmp DB in Diesel(this)?
// WIFI scanning as separate thread/routine & module
// scanning & sending can restore from state
#![feature(const_socketaddr)]

use chrono::{DateTime, Duration, Utc};

use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;


mod wifi;
mod client;
mod server;
mod db;
mod tests_common;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .with_line_number(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    dotenvy::from_filename("./config/app.env")
        .unwrap_or_else(|x| {
            panic!("Error loading env {}", x)
    });
    db::init_sqlx();

    let db_handle = tokio::spawn(async {
        db::establish_conn_sqlx().await.unwrap();
        db::init_sqlx().await.unwrap();
        db::test_sqlx().await.unwrap();
    });
    info!("spawned db as tokio task");

    let server_handle = tokio::spawn(async move {
        server::listen().await;
    });
    info!("spawned server as tokio task");

    let wifi_handle = tokio::spawn(async move {
        wifi::start().await;
    });
    info!("NOT IMPLEMENTED (stub) spawned wifi scanner as tokio task");

    Ok(())
}

async fn status() -> anyhow::Result<String> {
    let s = format!("Status info from main.rs {}\n {}\n{}\n",
        Utc::now().to_string(),
        db::status().await?, db::status().await?);
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::{assert_ok, assert_elapsed, assert_pending};

    #[tokio::test]
    async fn test_get_status() -> anyhow::Result<()> {
        let s = client::get_status().unwrap();
        info!("Got status '{}'", s);
        assert_eq!(s, "something".to_string());
        Ok(())
    }

    async fn test_post_update() -> anyhow::Result<()> {
        let s = client::post_update("Test123".to_string());
        let ss = assert_ok!(s);
        info!("got response '{}'", ss);
        assert_eq!(ss, "Response template:");
        Ok(())
    }

}