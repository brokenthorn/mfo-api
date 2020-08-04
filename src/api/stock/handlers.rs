//! Stock resource API handlers.

use tide::{Body, Request, Response, Result, StatusCode};

/// Handler function for __GET__ `/stock`
pub async fn get_stock(req: Request<crate::api::context::Context>) -> Result<Response> {
    // TODO: Pass data_curenta and data_ultima_actualizare from request parameters or the current defaults.
    let now_datetime = chrono::Local::now().format("%Y%m%d %H:%M:%S").to_string();
    let data_curenta = now_datetime.as_ref();
    let data_ultima_actualizare = "19000101";
    let api_response = req
        .state()
        .db
        .sp_bpws_web_get_stoc(data_ultima_actualizare, data_curenta)
        .await?;
    let response = Response::builder(StatusCode::Ok)
        .body(Body::from_json(&api_response)?)
        .build();
    Ok(response)
}
