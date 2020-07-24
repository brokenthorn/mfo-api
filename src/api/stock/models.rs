use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct StockLineItem {
    pub zona: String,
    pub id_articol: i32,
    pub cant_nr_buc: i32,
    pub pret_amanunt: f32,
    pub pret_achizitie: f32,
    pub id_locatie: i32,
    pub cantitate_ut: i32,
    pub lot: String,
    pub bbd: String,
}
