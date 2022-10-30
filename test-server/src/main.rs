use test_server::Exception;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Exception> {
    test_server::run().await?;
    Ok(())
}
