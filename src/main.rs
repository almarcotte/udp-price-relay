mod server;
mod client;
mod config;
mod message;

use std::env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = config::load();

    if args.len() > 1 && args[1] == "client" {
        println!("🔗 Starting client, listening on {}", config.listen_addr);
        client::start(&config.server_addr, &config.listen_addr).await?;
    } else {
        let config = config::load();
        println!("🖥️  Starting UDP relay server on {}...", config.server_addr);
        server::run(&config.server_addr).await?;
    }

    Ok(())
}
