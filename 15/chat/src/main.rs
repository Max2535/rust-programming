mod server;
mod client;

#[tokio::main]
async fn main() {
    let mode = std::env::args().nth(1).unwrap_or("server".into());

    if mode == "server" {
        server::run_server().await;
    } else {
        client::run_client().await;
    }
}
