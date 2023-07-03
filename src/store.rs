use reqwest::StatusCode;
use sqlx::{PgConnection, Connection};
use futures::TryStreamExt;
use sqlx::Row;
use sqlx::Column;
use sqlx::ValueRef;
use sqlx::Executor;

use crate::checker::ErrorDetails;
use crate::checker::ErrorType;
use crate::checker::SuccessDetails;
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

pub async fn add_health_check(conn: &mut PgConnection, result: &UriResult) {
        let success = match result.result {
            Ok(_) => true,
            Err(_) => false,
        };
        let status = match &result.result {
            Ok(i) => i.status,
            Err(i) => i.status,
        };
        let network_error = match &result.result {
            Ok(_) => false,
            Err(e) => {
                match e.etype {
                    ErrorType::NetworkError => true,
                    ErrorType::ServerError => false
                }
            }   
        };
        conn.execute(
            sqlx::query("INSERT INTO results (uri, request_time, success, network_error, status) VALUES (
                    $1,
                    $2,
                    $3,
                    $4,
                    $5
                )")
                .bind(&result.uri)
                .bind(result.time)
                .bind(success)
                .bind(network_error)
                .bind(i32::from(status))
        ).await.unwrap();
}

pub async fn ensure_setup(conn: &mut PgConnection) {
    conn.execute(sqlx::query("
        CREATE TABLE IF NOT EXISTS endpoints (
            id SERIAL PRIMARY KEY,
            uri TEXT NOT NULL
        );
    ")).await.unwrap();    

    conn.execute(sqlx::query("
        CREATE TABLE IF NOT EXISTS results (
            id SERIAL PRIMARY KEY,
            uri TEXT NOT NULL,
            request_time timestamp NOT NULL,
            success boolean NOT NULL,
            network_error boolean DEFAULT false,
            status integer
        );
    ")).await.unwrap();
}
