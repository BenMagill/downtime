use std::fmt::{Debug, self, Display, Formatter};

use reqwest::StatusCode;

use crate::store::{add_health_check, get_connection};

pub struct SuccessDetails {
    pub status: u16,
}

#[derive(Debug)]
pub enum ErrorType {
    NetworkError,
    ServerError,
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ErrorType::NetworkError => write!(f, "Network Error"),
            ErrorType::ServerError => write!(f, "Server Error")
        }
    }
}

pub struct ErrorDetails {
    pub etype: ErrorType,
    pub status: u16,
}

pub async fn check_endpoint(uri: &String) -> Result<SuccessDetails, ErrorDetails>{
    println!("Checking uri: {}", uri);
    let resp = reqwest::get(uri)
        .await;

    return match resp {
        Ok(res) => {
            let status = res.status();

            println!("Got status {} from endpoint", status.as_u16());            

            match status.as_u16() {
                100..=399 => {
                    Ok(SuccessDetails {
                        status: status.as_u16(),
                    })
                }
                _ => {
                    Err(ErrorDetails {
                        etype: ErrorType::ServerError,
                        status: status.as_u16()
                    })

                }
            }
        },
        Err(_) => {
            println!("Failed to call endpoint");

            Err(ErrorDetails {
                etype: ErrorType::NetworkError,
                status: 0,
            })
        }
    };
}

pub struct UriRecord {
    pub id: u32,
    pub uri: String
}

pub struct UriResult {
    pub id: u32,
    pub uri: String,
    pub time: chrono::DateTime<chrono::Utc>, 
    pub result: Result<SuccessDetails, ErrorDetails>, 
}

pub async fn check_all(uris: &Vec<UriRecord>) -> Vec<UriResult> {
    let mut results = Vec::new();

    for row in uris {
        let id = row.id;
        let uri = row.uri.clone();

        let result = check_endpoint(&uri).await;
        let res = UriResult {
            id,
            uri,
            time: chrono::Utc::now(),
            result
        };
        let mut conn = get_connection().await;
        add_health_check(&mut conn, &res).await;
        results.push(res);
    }
    return results;
}

pub fn debug_print_results(results: &Vec<UriResult>) {
    for result in results {
        match &result.result {
            Ok(i) => {
                println!("Ok - Status: {}", i.status);
            }
            Err(i) => {
                println!("{} - Status: {}", i.etype, &i.status);
            }
        }
    }
}
