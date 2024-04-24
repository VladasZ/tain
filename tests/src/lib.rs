use anyhow::Result;
use tain::request_containter;

#[tokio::test]
async fn test() -> Result<()> {
    request_containter().await?;

    Ok(())
}
