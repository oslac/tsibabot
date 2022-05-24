#[tokio::main]
async fn main() {
    if let Err(e) = tsiba::try_main().await {
        eprintln!("ERROR: {e}");
        std::process::exit(1)
    }
}
