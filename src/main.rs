use nproxy::start;

pub mod config;
pub mod handler;
pub mod nproxy;
pub mod worker;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    start().await;
    Ok(())
}
