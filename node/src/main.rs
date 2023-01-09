// TODO
// cross-compile it for radxa zero
// API Client
// tmp DB in Diesel(this)?
// WIFI scanning as separate thread/routine & module
// scanning & sending can restore from state

use client::{get_status, post_update};

mod wifi;
mod client;
mod db;

mod prelude {
    pub use crate::client::*;
}

fn main() {
    println!("Hello, world!");
    println!("Got status {}", get_status());
    println!("got response {}", post_update());
}
