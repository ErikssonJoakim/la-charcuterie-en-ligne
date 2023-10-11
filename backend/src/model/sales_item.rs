use mysql::{prelude::FromRow, FromRowError, Row};
use serde::Serialize;

#[derive(Serialize)]
pub struct SalesItem {
    pub id: u32,
    pub name: String,
    pub image: String,
    pub price: f32,
    pub quantity: u32,
    pub description: Option<String>,
    pub comparison_price: Option<f32>,
}

impl FromRow for SalesItem {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        Ok(SalesItem {
            id: row.get("id").ok_or(FromRowError(row.clone()))?,
            name: row.get("name").ok_or(FromRowError(row.clone()))?,
            image: row.get("image").ok_or(FromRowError(row.clone()))?,
            price: row.get("price").ok_or(FromRowError(row.clone()))?,
            quantity: row.get("quantity").ok_or(FromRowError(row.clone()))?,
            description: row.get("description"),
            comparison_price: row.get("comparison_price"),
        })
    }
}
