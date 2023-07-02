use std::fmt::{Debug, self, Display, Formatter};

use reqwest::StatusCode;

pub struct SuccessDetails {
    pub status: StatusCode,
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
    pub status: Option<StatusCode>,
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
                        status: status,
                    })
                }
                _ => {
                    Err(ErrorDetails {
                        etype: ErrorType::ServerError,
                        status: Some(status)
                    })

                }
            }
        },
        Err(_) => {
            println!("Failed to call endpoint");

            Err(ErrorDetails {
                etype: ErrorType::NetworkError,
                status: None,
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
    pub result: Result<SuccessDetails, ErrorDetails>, 
}

pub async fn check_all(uris: &Vec<UriRecord>) -> Vec<UriResult> {
    let mut results = Vec::new();

    for row in uris {
        let id = row.id;
        let uri = row.uri.clone();

        let result = check_endpoint(&uri).await;
        results.push(UriResult {
            id,
            uri,
            result
        })
    }
    return results;
}

pub fn debug_print_results(results: &Vec<UriResult>) {
    for result in results {
        match &result.result {
            Ok(i) => {
                println!("Ok - Status: {}", i.status.as_u16());
            }
            Err(i) => {
                let status = match &i.status {
                    Some(st) => st.as_str(),
                    None => "Unknown",
                };
                println!("{} - Status: {}", i.etype, status);
            }
        }
    }
}
