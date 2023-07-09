use crate::{
    db_connection::{Database, DatabaseConnection},
    model::SalesItem,
};
use actix_web::{
    get,
    web::{Json, Path},
    HttpResponse, Responder,
};
use mysql::{params, prelude::Queryable, DriverError, Error, PooledConn, Statement};
use std::io::{Error as StdIoError, ErrorKind::InvalidData};

struct StatementQuery {
    stmt: Statement,
    conn: PooledConn,
}

#[get("/{id}")]
pub async fn get_product(id: Path<u32>) -> impl Responder {
    let result = Database::connect()
        .and_then(|mut conn| {
            Ok(StatementQuery {
                stmt: conn.prep(
                    "
                    SELECT * 
                    FROM sales_item 
                    WHERE id=:id
                    ",
                )?,
                conn,
            })
        })
        .and_then(|mut query| {
            query
                .conn
                .exec_first(query.stmt, params! { "id" => id.into_inner() })
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
