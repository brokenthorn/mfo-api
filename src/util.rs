use crate::api::context::Context;
use tide::Server;

/// If RUST_LOG environment variable is not found, this function sets it to `RUST_LOG="warn,mfo_api=debug"`.
pub fn fix_rust_log_if_not_set() {
    if std::env::var("RUST_LOG").is_err() {
        tide::log::warn!("RUST_LOG was not set so logging might not be configured as expected!");
        std::env::set_var("RUST_LOG", "warn,mfo_api=debug");
    }
}

/// Creates the `mfo-api` server with the provided `context` state.
pub fn create_server(context: Context) -> Server<Context> {
    let mut app = Server::with_state(context);
    app = add_mfo_api_routes_to_server(app);
    app
}

/// Adds `mfo-api` routes to a Tide server.
pub fn add_mfo_api_routes_to_server(mut api: Server<Context>) -> Server<Context> {
    let mut stock_route = api.at("/stock");
    stock_route.get(|req| async move { crate::api::stock::handlers::get_stock(req).await });
    api
}
