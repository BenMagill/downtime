use clap::Parser;
mod store;
mod checker;
use store::*;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[clap(subcommand)]
    subcommand: SubCommand
}

#[derive(Parser, Debug)]
enum SubCommand {
    Add(AddSubCommand),
    List
}

#[derive(Parser, Debug)]
struct AddSubCommand {
    /// the url of an endpoint to be added
    #[arg(short, long )]
    url: String
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args.subcommand {
        SubCommand::Add(opts) => add(opts).await,
        SubCommand::List => list().await,
    };
}

async fn add(options: AddSubCommand) {
    println!("{}", options.url);

    let mut conn = get_connection().await;
    add_endpoint(&mut conn, options.url).await;
}

async fn list() {
    let mut conn = get_connection().await;
    let endpoints = get_endpoints(&mut conn).await;
    
    for endpoint in endpoints {
        println!("{}", endpoint.uri);
    }
}
