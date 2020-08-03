#![forbid(unsafe_code)]
#![forbid(deprecated_in_future)]
// #![warn(missing_docs)]

//! # MiniFarmOnline API
//!
//! This application is a backend Web API for [minifarmonline.ro](https://www.minifarmonline.ro/).
//!
//! Currently, it provides a single resource:
//!
//! ## Stock
//!
//! Provides information about which products are currently in stock.
//!
//! ### GET `/stock/`
//!
//! Get the current list of products in stock.
//!
//! > **Note**: This method is currently not optimized or cached!
//! >
//! > It queries the entire stock list each time it is called, so execution can take a while.
//! > At the moment of writing, this is around 20 seconds.

// #[macro_use]
// extern crate log;
// #[macro_use]
// extern crate anyhow;

pub mod api;
pub mod db;
pub mod util;

/// Application entry point.
#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    util::fix_rust_log_if_not_set();
    tide::log::with_level(tide::log::LevelFilter::Info);

    tide::log::info!("Application starting up.");
    let address = format!(
        "{}:{}",
        std::env::var("HOST").unwrap_or("0.0.0.0".to_string()),
        std::env::var("PORT").unwrap_or("8080".to_string())
    );

    tide::log::info!("Creating server.");
    let ctx = api::context::Context {
        db: db::mssql::MSSQL,
    };
    let app = api::create_mfo_api_server(ctx);

    tide::log::info!("Starting server on address {}", address);
    app.listen(address).await?;

    tide::log::info!("Application shutting down.");
    Ok(())
}
