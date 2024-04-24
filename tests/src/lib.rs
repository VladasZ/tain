use anyhow::Result;
use tain::request_container;

#[tokio::test]
async fn test() -> Result<()> {
    request_container().await?;

    Ok(())
}
