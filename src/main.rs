use nproxy::start;

pub mod config;
pub mod handlers;
pub mod nproxy;
pub mod utils;
pub mod worker;

#[tokio::main]
async fn main() {
    start().await;
}
