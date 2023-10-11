use crate::{
    db_connection::{Database, DatabaseConnection},
    model::SalesItem,
};
use actix_web::{
    get,
    web::{Json, Path},
    HttpResponse, Responder,
};
use mysql::{params, prelude::Queryable, DriverError, Error};
use std::io::{Error as StdIoError, ErrorKind::InvalidData};

#[get("")]
pub async fn get_products() -> impl Responder {
    let result: Result<Vec<SalesItem>, Error> = Database::connect().and_then(|mut conn| {
        let query = String::from("SELECT * FROM sales_item");
        conn.query(query)
    });

    match result {
        Ok(sales_items) => HttpResponse::Ok().json(Json(sales_items)),
        Err(e) => match e {
            Error::DriverError(e) => match e {
                DriverError::ConnectTimeout => HttpResponse::GatewayTimeout().finish(),
                DriverError::CouldNotConnect(None) => HttpResponse::ServiceUnavailable().finish(),
                _ => HttpResponse::InternalServerError().finish(),
            },
            Error::IoError(_) => HttpResponse::BadRequest().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

#[get("/{id}")]
pub async fn get_product(id: Path<u32>) -> impl Responder {
    let result = Database::connect()
        .and_then(|mut conn| {
            let stmt = conn.prep(
                "
                SELECT * 
                FROM sales_item 
                WHERE id=:id
                ",
            )?;

            conn.exec_first(stmt, params! { "id" => id.into_inner() })
        })
        .and_then(|sales_item: Option<SalesItem>| {
            sales_item.ok_or(Error::IoError(StdIoError::new(
                InvalidData,
                "No product found",
            )))
        });

    match result {
        Ok(sales_item) => HttpResponse::Ok().json(Json(sales_item)),
        Err(e) => match e {
            Error::DriverError(e) => match e {
                DriverError::ConnectTimeout => HttpResponse::GatewayTimeout().finish(),
                DriverError::CouldNotConnect(None) => HttpResponse::ServiceUnavailable().finish(),
                _ => HttpResponse::InternalServerError().finish(),
            },
            Error::IoError(e) => HttpResponse::BadRequest().body(e.to_string()),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}
