use super::models::StockLineItem;
use tide::{Body, Request, Response, Result, StatusCode};

// TODO: Write a fn result_to_response<T: IntoResponse, E: IntoResponse>(r: Result<T, E>) -> Response, in order to turn Results coming from DB getter fn's, into http Response's, and other such conversions.

/// Handler function for `GET /stock`
pub async fn get_stock(req: Request<crate::api::context::Context>) -> Result<Response> {
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

    let state = req.state();
    let conn_result = state.db.connect().await;
    match conn_result {
        Ok(mut conn) => {
            tide::log::info!("Got MSSQL connection: {:?}", conn);

            let current_date = chrono::Local::now().format("%Y%m%d %H:%M:%S").to_string();
            // NOTE: Due to a bug, SET NOCOUNT ON needs to be used when expecting results back from executing stored procedure, otherwise the driver gets back an affected rows value after which it stops processing the stream and does not get further result sets.
            match conn.query(
                "SET NOCOUNT ON; DECLARE @mesaj_eroare VARCHAR(255); EXEC [BizPharmaHO].[dbo].[spBPWSWebGetStoc] @DataUltimaActualizare = '19000101', @DataCurenta = @P1, @MesajEroare = @mesaj_eroare OUTPUT; SELECT @mesaj_eroare AS [mesaj_eroare];",
                &[&current_date],
            ).await {
                Ok(qr) => {
                    let results = qr.into_results().await?;
                    let trimmed_results = results.into_iter().map(|mut x| { x.truncate(2); x }).collect::<Vec<Vec<tiberius::Row>>>();
                    tide::log::info!("Got MSSQL query results (truncated results to first 2 rows): {:#?}", trimmed_results);
                }
                Err(err) => {
                    tide::log::error!("Error getting MSSQL query results: {:?}", err);
                    return Err(tide::Error::from_str(
                    StatusCode::InternalServerError,
                    err.to_string(),
                    ));
                }
            }
        }
        Err(err) => {
            tide::log::error!("Error getting MSSQL connection: {:?}", err);
            return Err(tide::Error::from_str(
                StatusCode::InternalServerError,
                err.to_string(),
            ));
        }
    }

    Ok(Response::builder(StatusCode::Ok)
        .body(Body::from_json(&stock_line_item).unwrap())
        .build())
}
