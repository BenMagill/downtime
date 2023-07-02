use sqlx::{PgConnection, Connection};
use futures::TryStreamExt;
use sqlx::Row;
use sqlx::Column;
use sqlx::ValueRef;
use sqlx::Executor;

use crate::checker::UriRecord;
use crate::checker::UriResult;

pub async fn get_connection() -> PgConnection {
        PgConnection::connect("postgresql://postgres:postgres@localhost:5432").await.unwrap()
}

pub async fn get_endpoints(conn: &mut PgConnection) -> Vec<UriRecord> {
        let mut rows = sqlx::query("SELECT * FROM endpoints;").fetch(conn);
        
        let mut uris: Vec<UriRecord> = Vec::new();
        while let Some(row) = rows.try_next().await.unwrap() {
        
            let id = row.get::<i32, _>("id");
            let uri = row.get::<String, _>("uri");
            uris.push(UriRecord {
                id: id.try_into().unwrap(),
                uri,
            })
        }

        uris
}

pub async fn add_endpoint(conn: &mut PgConnection, uri: String) {
        conn.execute(sqlx::query("INSERT INTO endpoints (uri) VALUES ($1);").bind(uri)).await.unwrap();
}

pub async fn add_health_check(conn: &mut PgConnection, result: UriResult) {
}

pub async fn ensure_setup(conn: &mut PgConnection) {
    conn.execute(sqlx::query("
        CREATE TABLE IF NOT EXISTS endpoints (
            id SERIAL PRIMARY KEY,
            uri TEXT
        );
    ")).await.unwrap();    

    conn.execute(sqlx::query("
        CREATE TABLE IF NOT EXISTS results (
            id SERIAL PRIMARY KEY,
            uri_id integer REFERENCES endpoints (id) NOT NULL,
            request_time timestamp NOT NULL,
            success boolean NOT NULL,
            network_error boolean DEFAULT false,
            status integer
        );
    ")).await.unwrap();
}
