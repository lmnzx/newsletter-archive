use newsletter::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run(std::net::TcpListener::bind("127.0.0.1:8000")?)?.await
}
