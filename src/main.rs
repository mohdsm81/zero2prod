use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:80").expect("Faild to bind to address and port 80");
    run(listener)?.await
}
