//! Stock model objects.

use crate::api::ApiResponse;
use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Serialize;
use tiberius::QueryResult;
use tide::{Error, Result, StatusCode};

#[derive(Debug, Serialize)]
pub struct StockLineItem {
    pub zona: String,
    pub id_articol: i32,
    pub cant_nr_buc: i32,
    pub pret_amanunt: Decimal,
    pub pret_achizitie: Decimal,
    pub id_locatie: i32,
    pub cantitate_ut: i32,
    pub lot: String,
    pub bbd: NaiveDateTime,
}

pub type Stock = Vec<StockLineItem>;

// TODO: move this function to the correct module:
pub async fn stock_query_result_to_api_response<'a>(
    qr: QueryResult<'a>,
) -> Result<ApiResponse<Stock, String>> {
    let resultsets = qr.into_results().await?;
    let (sp_execution_result, sp_output_result) = resultsets.split_at(1);
    let mut stock: Stock = vec![];

    // Resultset no. 1: stored procedure's main (first and only) resultset:

    if let Some(data) = sp_execution_result.first() {
        // we're using try_for_each because of its early return in case of errors:
        data.iter().try_for_each(|r| -> Result<()> {
            stock.push(StockLineItem {
                zona: r
                    .try_get::<&str, &str>("Zona")?
                    .ok_or(Error::from_str(
                        StatusCode::InternalServerError,
                        "Zona was null",
                    ))?
                    .into(),
                id_articol: r.try_get::<i32, &str>("IdArticol")?.ok_or(Error::from_str(
                    StatusCode::InternalServerError,
                    "IdArticol was null.",
                ))?,
                cant_nr_buc: r.try_get::<i32, &str>("CantNrBuc")?.ok_or(Error::from_str(
                    StatusCode::InternalServerError,
                    "CantNrBuc was null.",
                ))?,
                pret_amanunt: r
                    .try_get::<Decimal, &str>("PretAmanunt")?
                    .ok_or(Error::from_str(
                        StatusCode::InternalServerError,
                        "PretAmanunt was null.",
                    ))?,
                pret_achizitie: r.try_get::<Decimal, &str>("PretAchizitie")?.ok_or(
                    Error::from_str(StatusCode::InternalServerError, "PretAchizitie was null."),
                )?,
                id_locatie: r.try_get::<i32, &str>("IdLocatie")?.ok_or(Error::from_str(
                    StatusCode::InternalServerError,
                    "IdLocatie was null.",
                ))?,
                cantitate_ut: r
                    .try_get::<i32, &str>("CantitateUT")?
                    .ok_or(Error::from_str(
                        StatusCode::InternalServerError,
                        "CantUT was null.",
                    ))?,
                lot: r
                    .try_get::<&str, &str>("Lot")?
                    .ok_or(Error::from_str(
                        StatusCode::InternalServerError,
                        "Lot was null.",
                    ))?
                    .into(),
                bbd: r
                    .try_get::<NaiveDateTime, &str>("BBD")?
                    .ok_or(Error::from_str(
                        StatusCode::InternalServerError,
                        "BBD was null.",
                    ))?,
            });

            Ok(())
        })?;
    } else {
        // the resultset was not present, should not theoretically happen, but let's handle it:
        return Err(Error::from_str(
            StatusCode::InternalServerError,
            "There was no resultset for SQL sproc output.",
        ));
    }

    // Resultset no. 2: stored procedure OUTPUT variables:

    let mut errs: Vec<String> = vec![];

    if let Some(data) = sp_output_result.first() {
        // we're using try_for_each because of its early return in case of errors:
        data.iter().try_for_each(|r| -> Result<()> {
            let msg = r
                .try_get::<&str, &str>("mesaj_eroare")?
                .ok_or(Error::from_str(
                    StatusCode::InternalServerError,
                    "`mesaj_eroare` (SQL sproc OUTPUT) variable was null, it should be a string.",
                ))?
                .into();
            errs.push(msg);
            Ok(())
        })?;
    } else {
        // the resultset was not present, should not theoretically happen, but let's handle it:
        return Err(Error::from_str(
            StatusCode::InternalServerError,
            "There was no resultset for SQL sproc `mesaj_eroare` output.",
        ));
    }

    Ok(ApiResponse {
        data: if stock.is_empty() { None } else { Some(stock) },
        errors: if errs.is_empty() { None } else { Some(errs) },
    })
}
