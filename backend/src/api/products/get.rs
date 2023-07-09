use crate::{db_connection::{Database, DatabaseConnection}, model::SalesItem};
use actix_web::{web::Json, get, HttpResponse, Responder};
use mysql::{Row, prelude::Queryable, Error, DriverError};
use std::io::{Error as StdIoError, ErrorKind::InvalidData};

#[get("")]
pub async fn get_products() -> impl Responder {
    let result: Result<Vec<SalesItem>, Error> = Database::connect()
    .and_then(|mut conn | {
        let query = String::from("SELECT * FROM sales_item");
        
        conn.query_map(query, | row: Row | row)
    })
    .and_then(|rows | rows.into_iter().map(| row | 
        Ok(SalesItem { 
            id: row.get("id").ok_or(Error::IoError(StdIoError::new(InvalidData, "Value: id is missing")))?,
            name: row.get("name").ok_or(Error::IoError(StdIoError::new(InvalidData, "Value: name is missing")))?,
            image: row.get("image").ok_or(Error::IoError(StdIoError::new(InvalidData, "Value: image is missing")))?,
            price: row.get("price").ok_or(Error::IoError(StdIoError::new(InvalidData, "Value: price is missing")))?,
            quantity: row.get("quantity").ok_or(Error::IoError(StdIoError::new(InvalidData, "Value: quantity is missing")))?,
            description: row.get("description"),
            comparison_price: row.get("comparison_price")
        })
    ).collect());
        
    match result {
        Ok(sales_items) => HttpResponse::Ok().json(Json(sales_items)),
        Err(e) => match e {
           Error::DriverError(e) => match e {
               DriverError::ConnectTimeout => HttpResponse::GatewayTimeout().finish(),
               DriverError::CouldNotConnect(None) => HttpResponse::ServiceUnavailable().finish(),
               _ => HttpResponse::InternalServerError().finish()
           },
           Error::IoError(_) => HttpResponse::BadRequest().finish(),
           _ => HttpResponse::InternalServerError().finish()
       } 
    }
}

