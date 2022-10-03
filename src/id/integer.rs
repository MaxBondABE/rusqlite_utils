use rusqlite::{
    types::{FromSql, ToSqlOutput},
    Row, ToSql,
};
use std::marker::PhantomData;

use super::Id;

/// Represents a column named `id` stored as a SQLite `INTEGER`.
/// The type parameter allows it to be bound to a particular
/// table, to provide type safety.
pub struct IntegerId<T>(i64, PhantomData<T>);
impl<'stmt, T> Id<'stmt> for IntegerId<T> {}

// The following are normally implemented via derive; however, this
// would put unneccessary requirements on T.

impl<T> Copy for IntegerId<T> {}
impl<T> Clone for IntegerId<T> {
    fn clone(&self) -> Self {
        Self(self.0, PhantomData)
    }
}
impl<T> std::fmt::Debug for IntegerId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("IntegerId({})", self.0))
    }
}
impl<T> Eq for IntegerId<T> {}
impl<T> PartialEq for IntegerId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T> Ord for IntegerId<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
impl<T> PartialOrd for IntegerId<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl<T> std::hash::Hash for IntegerId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}
impl<T> ToSql for IntegerId<T> {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.0))
    }
}
impl<T> FromSql for IntegerId<T> {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let v: i64 = value.as_i64()?;
        Ok(Self(v, PhantomData))
    }
}
impl<'stmt, T> TryFrom<&Row<'stmt>> for IntegerId<T> {
    type Error = rusqlite::Error;

    fn try_from(value: &Row<'stmt>) -> Result<Self, Self::Error> {
        Ok(Self(value.get("id")?, PhantomData))
    }
}

#[cfg(test)]
mod test {
    use rusqlite::Connection;

    use super::*;

    #[test]
    fn insert_and_retrieve_id() {
        let db = Connection::open_in_memory().expect("Failed to open connection");
        type FooId = IntegerId<()>;

        db.execute(
            "create table foo( id integer primary key autoincrement )",
            (),
        )
        .expect("Failed to create table");
        let res = db.query_row("insert into foo default values returning *", (), |row| {
            let v: FooId = row.try_into()?;
            Ok(v)
        });
        assert!(
            res.is_ok(),
            "Failed to retrieve id from database: {:?}",
            res
        );
    }

    #[test]
    fn select_by_id() {
        let db = Connection::open_in_memory().expect("Failed to open connection");
        type FooId = IntegerId<()>;

        db.execute(
            "create table foo( id integer primary key autoincrement, bar integer )",
            (),
        )
        .expect("Failed to create table");
        let res = db.query_row("insert into foo(bar) values(10) returning *", (), |row| {
            let v: FooId = row.try_into()?;
            Ok(v)
        });
        assert!(
            res.is_ok(),
            "Failed to retrieve id from database: {:?}",
            res
        );
        let id = res.unwrap();

        let res = db.query_row("select bar from foo where id = ?", (id,), |row| {
            let v: i64 = row.get("bar")?;
            Ok(v)
        });
        assert!(
            res.is_ok(),
            "Failed to retrieve id from database: {:?}",
            res
        );
        let value = res.unwrap();
        assert_eq!(value, 10);
    }

    #[test]
    fn retrieve_id_as_part_of_struct() {
        let db = Connection::open_in_memory().expect("Failed to open connection");
        type FooId = IntegerId<Foo>;
        #[derive(PartialEq, Eq, Debug)]
        struct Foo {
            id: FooId,
            bar: i64,
        }
        impl<'stmt> TryFrom<&Row<'stmt>> for Foo {
            type Error = rusqlite::Error;

            fn try_from(value: &Row<'stmt>) -> Result<Self, Self::Error> {
                Ok(Self {
                    id: value.get("id")?,
                    bar: value.get("bar")?,
                })
            }
        }

        db.execute(
            "create table foo( id integer primary key autoincrement, bar integer )",
            (),
        )
        .expect("Failed to create table");
        let res = db.query_row("insert into foo(bar) values(10) returning *", (), |row| {
            let v: Foo = row.try_into()?;
            Ok(v)
        });
        assert!(
            res.is_ok(),
            "Failed to retrieve id from database: {:?}",
            res
        );
    }
}
