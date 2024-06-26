#![cfg(test)]

use tain::{Postgres, PostgresArc};

fn get_postgres() -> PostgresArc {
    Postgres::sokolikcik(|| {
        Postgres::default()
            .db("db_name")
            .user("user")
            .password("password")
            .data("path/to/data")
            .start_container()
    })
}

fn test_postgres(pg: PostgresArc) {
    let port = pg.get_host_port_ipv4(5432);

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
}

#[test]
fn test0() {
    test_postgres(get_postgres())
}

#[test]
fn test1() {
    test_postgres(get_postgres())
}

#[test]
fn test2() {
    test_postgres(get_postgres())
}

#[test]
fn test3() {
    test_postgres(get_postgres())
}

#[test]
fn test4() {
    test_postgres(get_postgres())
}

#[test]
fn test5() {
    test_postgres(get_postgres())
}

#[test]
fn test6() {
    test_postgres(get_postgres())
}

#[test]
fn test7() {
    test_postgres(get_postgres())
}

#[test]
fn test8() {
    test_postgres(get_postgres())
}

#[test]
fn test9() {
    test_postgres(get_postgres())
}
