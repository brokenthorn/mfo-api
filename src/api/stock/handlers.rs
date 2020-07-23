use super::models::StockLineItem;
use tide::{Body, Request, Response, Result, StatusCode};

// TODO: Write a fn result_to_response<T: IntoResponse, E: IntoResponse>(r: Result<T, E>) -> Response, in order to turn Results coming from DB getter fn's, into http Response's, and other such conversions.

/// Handler function for `GET /stock`
pub(crate) async fn get_stock(_req: Request<crate::api::context::Context>) -> Result<Response> {
    tide::log::info!("Executing handler function for `GET /stock`");
    let stock_line_item = StockLineItem {
        zona: String::from("A"),
        id_articol: 1,
        cant_nr_buc: 1,
        pret_amanunt: 1.1,
        pret_achizitie: 0.1,
        id_locatie: 1,
        cantitate_ut: 30,
        lot: String::from("lot"),
        bbd: String::from("bbd"),
    };
    Ok(Response::builder(StatusCode::Ok)
        .body(Body::from_json(&stock_line_item).unwrap())
        .build())
}
