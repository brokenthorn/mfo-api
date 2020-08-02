//! Stock resource API handlers.

use super::models::stock_query_result_to_api_response;

use tide::{Body, Request, Response, Result, StatusCode};

// TODO: Write a fn result_to_response<T: IntoResponse, E: IntoResponse>(r: Result<T, E>) -> Response, in order to turn Results coming from DB getter fn's, into http Response's, and other such conversions.

/// Handler function for __GET__ `/stock`
pub async fn get_stock(req: Request<crate::api::context::Context>) -> Result<Response> {
    let state = req.state();

    // TODO: Extract database logic into separate function

    let conn_result = state.db.connect().await;

    match conn_result {
        Ok(mut conn) => {
            let current_date = chrono::Local::now().format("%Y%m%d %H:%M:%S").to_string();

            // REF: Due to a known bug, SET NOCOUNT ON needs to be used when expecting results back from executing stored procedure, otherwise the driver gets back an `affected rows` value after which it stops processing the stream and does not get further result sets.
            match conn.query(
                "SET NOCOUNT ON; DECLARE @mesaj_eroare VARCHAR(255); EXEC [BizPharmaHO].[dbo].[spBPWSWebGetStoc] @DataUltimaActualizare = '19000101', @DataCurenta = @P1, @MesajEroare = @mesaj_eroare OUTPUT; SELECT @mesaj_eroare AS [mesaj_eroare];",
                &[&current_date],
            ).await {
                Ok(qr) => {
                    match stock_query_result_to_api_response(qr).await {
                        Ok(api_response) => {
                            return Ok(
                                Response::builder(StatusCode::Ok)
                                .body(Body::from_json(&api_response).unwrap()) // TODO: Handle unwrap() call and return proper error.
                                .build()
                            );
                        }
                        Err(error) => {
                            tide::log::error!("Failed to get stock: {:?}", error);
                            // TODO: Build custom error type for these returns, research if possible to use anyhow and convert to tide errors.
                            return Err(
                                tide::Error::from_str(
                                    StatusCode::InternalServerError,
                                    error.to_string()
                                )
                            );
                        }
                    }

                    // let results = qr.into_results().await?;
                    // let (data, mesaj_eroare) = results.split_at(1);
                    // let trimmed_results = results.into_iter().map(|mut x| { x.truncate(2); x }).collect::<Vec<Vec<tiberius::Row>>>();
                    // tide::log::info!("Got MSSQL query results (truncated results to first 2 rows): {:#?}", trimmed_results);
                }
                Err(error) => {
                    tide::log::error!("Error getting MSSQL query results: {:?}", error);
                    return Err(tide::Error::from_str(
                    StatusCode::InternalServerError,
                    error.to_string(),
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
}
