mod checker;
mod store;

use std::error::Error;
use checker::{UriRecord, check_all, debug_print_results};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let uris = vec![
        UriRecord {
            id: 1,
            uri: "http://localhost:8080/health".to_string(),
        },
        UriRecord {
            id: 2,
            uri: "https://localhost:8080/health".to_string(),
        },
    ];

    let res = check_all(uris).await;

    debug_print_results(&res);
    
    store::run().await;

    Ok(())
}
