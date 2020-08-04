//! API implementation.

pub mod auth;
pub mod context;
pub mod stock;

use serde::Serialize;
use tide::Server;

use crate::api::context::Context;

/// Generic serializable API response type with two optional fields: `data` and `error`.
#[derive(Serialize)]
pub struct ApiResponse<D, E>
where
    D: Serialize + Sync + Send,
    E: Serialize + Sync + Send,
{
    /// Requested data.
    pub data: Option<D>,
    /// Any errors that might have occurred.
    pub errors: Option<Vec<E>>,
}

/// Creates the `mfo-api` server with the provided `context` state.
pub fn create_mfo_api_server(context: Context) -> Server<Context> {
    let compress_middleware = tide_compress::CompressMiddleware::with_threshold(1024);
    let mut app = Server::with_state(context);
    app.with(compress_middleware);
    add_mfo_api_routes_to_server(&mut app);
    app
}

/// Adds `mfo-api` routes to a Tide server.
fn add_mfo_api_routes_to_server(api: &mut Server<Context>) {
    let mut stock_route = api.at("/stock");
    stock_route.get(|req| async move { crate::api::stock::handlers::get_stock(req).await });
}
