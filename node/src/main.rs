// TODO
// cross-compile it for radxa zero
// API Client
// tmp DB in Diesel(this)?
// WIFI scanning as separate thread/routine & module
// scanning & sending can restore from state

mod wifi;
mod client;
mod db;

mod prelude {
    // pub use crate::client::HMMMM
}

fn main() {
    println!("Hello, world!");
}
