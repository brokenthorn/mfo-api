use tide::prelude::json;
use tide::{Request, Response, Result, StatusCode};

/// Custom handler function for invalid resources (404: NOT FOUND)
pub async fn handle_404(req: Request<crate::api::context::Context>) -> Result<Response> {
    tide::log::info!("Path not found (404): {:?}", req);
    Ok(Response::builder(StatusCode::NotFound)
        .body(json!({ "error": "NOT_FOUND", "code": 404 }))
        .build())
}
