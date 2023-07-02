use mysql::{ Error, Pool, PooledConn, OptsBuilder};
use std::{env, result::Result};

pub struct Database;
pub trait DatabaseConnection {
     fn connect() -> Result<PooledConn, Error>;
}

impl DatabaseConnection for Database {
    
    fn connect() -> Result<PooledConn, Error> {
        let username = env::var("DB_USERNAME");
        let password = env::var("DB_PASSWORD");
        let host = env::var("DB_HOST");
        let port = env::var("DB_PORT");
        let db_name = env::var("DB_DATABASE_NAME");

        let opts = OptsBuilder::new()
        .user(username.ok())
            .pass(password.ok())
            .ip_or_hostname(host.ok())
            .tcp_port(
                port
                .unwrap_or_else(| error| panic!("{}", error))
                .parse()
                .unwrap_or_else(|error| panic!("{}", error))
            )
            .db_name(db_name.ok())
            .prefer_socket(false);

        let pool = Pool::new(opts)?;
        pool.get_conn()
    }
}