mod models;

use tracing::{info, debug, error};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use once_cell::sync::OnceCell;
use std::env;
use models::*;
pub use models::{WifiEntry};

const GET_LIMIT: i32 = 0;

static DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();
static WIFI_TABLE: OnceCell<String> = OnceCell::new();
// static SUBSCRIBER: OnceCell<FmtSubscriber> = OnceCell::new();

// pub fn init(s: FmtSubscriber) {
//     tracing::subscriber::set_global_default(s)
//         .expect("setting default subscriber failed");
// }

#[tracing::instrument]
pub async fn establish_conn_sqlx() -> anyhow::Result<()> {
    error!("test error");
    info!("tracing whatever");

    tracing::info_span!("opening DB pool");
    info!("in DB pool span");

    let database_url = env::var("DATABASE_URL").expect("DB URL must be set");
    debug!("{}", database_url);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await?;
    DB_POOL.set(pool).expect("Couldn't set DB_POOL OnceCell for");
    info!("at the end of DB pool span");
    info!("finished setting up database connection");
    Ok(())
}

pub async fn init_sqlx() -> anyhow::Result<()> {
    match env::var("DB_WIFI_TABLE") {
        Ok(x) => { WIFI_TABLE.set(x).expect("Couldn't set WIFI_TABLE"); },
        _ => { WIFI_TABLE.set("wifi".to_string()).expect("Couldn't set WIFI_TABLE default value"); },
    }

    let p = DB_POOL.get().unwrap();
    let w = WIFI_TABLE.get().unwrap();

    let _o = sqlx::query(r"
        CREATE TABLE IF NOT EXISTS testtb (
            id      BIGSERIAL PRIMARY KEY,
            msg     TEXT NOT NULL,
            counter INTEGER,
            json    JSONB )
        ")
        .execute(p).await?;

    let _o = sqlx::query(r"
        CREATE TABLE IF NOT EXISTS ?
        (
            id      BIGSERIAL PRIMARY KEY,
            lat     INTEGER NOT NULL,
            lon     INTEGER NOT NULL,
            ssid    TEXT NOT NUll,
        )
        ")
        .bind(w)
        .execute(p).await?;
    Ok(())
}

pub async fn test_sqlx() -> anyhow::Result<()> {
    let p = DB_POOL.get().unwrap();

    let o = sqlx::query("SELECT * FROM testtb")
        .execute(p).await?;
    debug!("o: {:?}", o);

    sqlx::query("UPDATE testtb SET msg=? WHERE id=?")
        .bind("test_sqlx change")
        .bind(1)
        .execute(p).await?;

    sqlx::query("DELETE FROM testtb WHERE id=?")
        .bind(1)
        .execute(p).await?;

    sqlx::query("DELETE FROM testtb WHERE id=?")
        .bind(1)
        .execute(p).await?;

    Ok(())
}

pub async fn status() -> anyhow::Result<String> {
    let p = DB_POOL.get().unwrap();
    sqlx::query("SELECT COUNT(*) FROM ?")
        .bind("")
        .execute(p).await?;
    Ok("DB status".to_string())
}

pub async fn get_entries() -> anyhow::Result<Vec<WifiEntry>> {
    let (p, w) = (DB_POOL.get().unwrap(), WIFI_TABLE.get().unwrap());
    let o = sqlx::query("SELECT * FROM ? LIMIT ?")
        .bind(w)
        .bind(GET_LIMIT)
        .execute(p).await?;
    info!("get entries succeeded, output: {:?}", o);

    Ok(Vec::new())
}

pub async fn remove_entries(e: Vec<WifiEntry>) -> anyhow::Result<()>{
    let (p, w) = (DB_POOL.get().unwrap(), WIFI_TABLE.get().unwrap());
    let o = sqlx::query("DELETE ? FROM ?")
        .bind(w)
        .execute(p).await?;
    info!("entries deleted, output: {:?}", o);
    Ok(())
}

pub async fn entries_available() -> anyhow::Result<usize> {
    let (p, w) = (DB_POOL.get().unwrap(), WIFI_TABLE.get().unwrap());
    let o = sqlx::query("SELECT COUNT(*) FROM ?")
        .bind(w)
        .execute(p).await?;
    info!("entries available are {:?}", o);
    Ok(0usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;
    // use tokio::test;

    static SETUP: Once = Once::new();

    async fn setup_global() {
        SETUP.call_once(|| {
            init_sqlx();
        });
    }

    #[tokio::test]
    async fn test_serialize() {
        setup_global().await;

    }

    #[test]
    fn test_deserialize() {
        tokio_test::block_on(setup_global());

    }
    
}