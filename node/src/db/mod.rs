mod models;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use once_cell::sync::OnceCell;
use std::env;
use std::path::{Path};
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;

static DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();
static SUBSCRIBER: OnceCell<FmtSubscriber> = OnceCell::new();

pub fn establish_conn_diesel() -> PgConnection {
    dotenvy::from_filename("./config/app.env")
        .unwrap_or_else(|x| {
            panic!("Error loading env {}", x)
    });
    env::vars().for_each(|(k, v)| println!("{} is {}", k, v));
    let database_url = env::var("DATABASE_URL").expect("DB URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn init(s: FmtSubscriber) {
    tracing::subscriber::set_global_default(s)
        .expect("setting default subscriber failed");
}

pub async fn establish_conn_sqlx() -> anyhow::Result<()> {

    info!("tracing whatever");

    dotenvy::from_filename("./config/app.env")
        .unwrap_or_else(|x| {
            panic!("Error loading env {}", x)
    });
    // env::vars().for_each(|(k, v)| println!("{} is {}", k, v));
    let database_url = env::var("DATABASE_URL").expect("DB URL must be set");
    println!("{}", database_url);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await?;
    DB_POOL.set(pool).expect("Couldn't set DB_POOL OnceCell for");
    Ok(())
}

pub async fn init_sqlx() -> anyhow::Result<()> {
    let p = DB_POOL.get().unwrap();

    let o = sqlx::query(r"
CREATE TABLE IF NOT EXISTS testtb
(
    id      BIGSERIAL PRIMARY KEY,
    msg     TEXT NOT NULL,
    counter INTEGER,
    json    JSONB
)
")
        .execute(p).await?;
    Ok(())
}

pub async fn test_sqlx() -> anyhow::Result<()> {
    let p = DB_POOL.get().unwrap();

    let o = sqlx::query("SELECT * FROM testtb")
        .execute(p).await?;
    println!("o: {:?}", o);

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