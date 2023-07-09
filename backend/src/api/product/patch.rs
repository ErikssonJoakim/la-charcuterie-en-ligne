use crate::db_connection::{Database, DatabaseConnection};
use actix_web::{
    patch,
    web::{Json, Path},
    HttpResponse, Responder,
};
use mysql::{params, prelude::Queryable, DriverError, Error, PooledConn, Statement};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Data {
    delta: i32,
}
struct StatementQuery {
    stmt: Statement,
    conn: PooledConn,
}

#[patch("/{id}")]
pub async fn update_product_count(id: Path<u32>, data: Json<Data>) -> impl Responder {
    let result = Database::connect()
        .and_then(|mut conn| {
            conn.prep(
                "UPDATE sales_item 
            SET quantity = quantity + :delta
            WHERE id=:id",
            )
            .and_then(|stmt| Ok(StatementQuery { stmt, conn }))
        })
        .and_then(|mut query| {
            query.conn.exec_drop(
                query.stmt,
                params! { "id" => id.into_inner(), "delta" => data.delta },
            )
        });

    match result {
        Ok(_) => HttpResponse::Ok(),
        Err(e) => match e {
            Error::MySqlError(e) => match e.code {
                1690 => HttpResponse::Conflict(), // out of range
                _ => HttpResponse::InternalServerError(),
            },
            Error::DriverError(e) => match e {
                DriverError::ConnectTimeout => HttpResponse::GatewayTimeout(),
                DriverError::CouldNotConnect(None) => HttpResponse::ServiceUnavailable(),
                _ => HttpResponse::InternalServerError(),
            },
            Error::FromValueError(_) => HttpResponse::BadRequest(),
            _ => HttpResponse::InternalServerError(),
        },
    }
}
