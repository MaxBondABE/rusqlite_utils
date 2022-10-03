#![cfg(test)]

use macros::TryFromRow;
use rusqlite::Connection;

#[test]
fn smoke_test() {
    #[derive(TryFromRow, Debug)]
    struct Foo {
        a: i64,
    }
}

#[test]
fn retrieve_row() {
    #[derive(TryFromRow, Debug)]
    struct Foo {
        a: i64,
    }

    let db = Connection::open_in_memory().expect("failed to open in-memory db");
    db.execute("create table foo(a integer)", ())
        .expect("failed to create table");
    db.execute("insert into foo(a) values (10)", ())
        .expect("failed to insert row");

    let res: rusqlite::Result<Foo> =
        db.query_row("select * from foo limit 1", (), |row| row.try_into());
    assert!(res.is_ok(), "Failed to retrieve row: {:?}", res);
}
