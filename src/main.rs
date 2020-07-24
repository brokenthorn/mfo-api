#![forbid(unsafe_code)]
#![forbid(deprecated_in_future)]
// #![warn(missing_docs)]

// #[macro_use]
// extern crate log;
// #[macro_use]
// extern crate anyhow;

pub mod api;
pub mod db;
pub mod util;

use db::mssql::MSSQL;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();
    util::fix_rust_log_if_not_set();
    tide::log::info!("Starting up.");

    let address = format!(
        "{}:{}",
        std::env::var("HOST").unwrap_or("0.0.0.0".to_string()),
        std::env::var("PORT").unwrap_or("8080".to_string())
    );
    let ctx = api::context::Context { db: MSSQL };
    let app = util::create_server(ctx);
    app.listen(address).await?;

    tide::log::info!("Shutting down.");
    Ok(())
}
