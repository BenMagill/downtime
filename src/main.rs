mod checker;
mod store;

use std::error::Error;
use checker::{UriRecord, check_all, debug_print_results};
use store::{ensure_setup, get_connection, get_endpoints};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut conn = get_connection().await;   
    ensure_setup(&mut conn).await;

    let uris = store::get_endpoints(&mut conn).await;
    let res = check_all(&uris).await;

    debug_print_results(&res);
    for i in uris {
        println!("{}, {}", i.id, i.uri);
    }

    Ok(())
}
