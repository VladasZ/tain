use anyhow::Result;
use tain::Postgres;

#[test]
fn test() -> Result<()> {
    let container = Postgres::default()
        .db("db_name")
        .user("user")
        .password("password")
        .data("path/to/data")
        .start_container();

    let port = container.get_host_port_ipv4(5432);

    let mut conn = postgres::Client::connect(
        &format!("postgres://user:password@localhost:{port}/db_name"),
        postgres::NoTls,
    )
    .unwrap();

    let rows = conn.query("SELECT 1 + 1", &[]).unwrap();
    assert_eq!(rows.len(), 1);

    let first_row = &rows[0];
    let first_column: i32 = first_row.get(0);
    assert_eq!(first_column, 2);

    Ok(())
}
