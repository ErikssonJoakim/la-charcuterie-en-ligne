use serde::Serialize;

#[derive(Serialize)]
pub struct SalesItem {
    pub id: u32,
    pub name: String,
    pub image: String,
    pub price: f32,
    pub quantity: u32,
    pub description: Option<String>,
    pub comparison_price: Option<f32>
}