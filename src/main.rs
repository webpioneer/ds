mod client;
mod server;

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <server|client>", args[0]);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "server" => server::run_server().await?,
        "client" => client::run_client().await?,
        _ => {
            eprintln!("Invalid argument: {}", args[1]);
            std::process::exit(1);
        }
    }

    Ok(())
}
