use anyhow::Result;
use tain::Postgres;

#[tokio::test]
async fn test() -> Result<()> {
    let _container = Postgres::default()
        .db("db_name")
        .user("user")
        .password("password")
        .port(1111)
        .data("path/to/data")
        .start_container();

    Ok(())
}
