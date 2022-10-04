use rusqlite::{
    types::{FromSql, FromSqlError, ToSqlOutput},
    ToSql,
};
use serde::{de::DeserializeOwned, Serialize};

/// Represents a BSON-encoded column value stored as a SQLite `BLOB`. T should implement
/// serde Serialize & DeserializeOwned.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BsonObject<T>(T);
impl<T> BsonObject<T> {
    pub fn new(v: T) -> Self {
        Self(v)
    }
    pub fn unwrap(self) -> T {
        self.0
    }
}
impl<T: Serialize> ToSql for BsonObject<T> {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let conversion_res = bson::ser::to_vec(&self.0);
        if let Ok(v) = conversion_res {
            Ok(ToSqlOutput::from(v))
        } else {
            Err(rusqlite::Error::ToSqlConversionFailure(Box::new(
                conversion_res.err().unwrap(),
            )))
        }
    }
}
impl<T: DeserializeOwned> FromSql for BsonObject<T> {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let conversion_res = bson::de::from_slice::<T>(value.as_blob()?);
        if let Ok(v) = conversion_res {
            Ok(Self::new(v))
        } else {
            Err(FromSqlError::InvalidType)
        }
    }
}

/// Represents a JSON-encoded column value stored as a SQLite `TEXT`. T should implement
/// serde Serialize & DeserializeOwned.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct JsonObject<T>(T);
impl<T> JsonObject<T> {
    pub fn new(v: T) -> Self {
        Self(v)
    }
    pub fn unwrap(self) -> T {
        self.0
    }
}
impl<T: Serialize> ToSql for JsonObject<T> {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let conversion_res = serde_json::to_string(&self.0);
        if let Ok(v) = conversion_res {
            Ok(ToSqlOutput::from(v))
        } else {
            Err(rusqlite::Error::ToSqlConversionFailure(Box::new(
                conversion_res.err().unwrap(),
            )))
        }
    }
}
impl<T: DeserializeOwned> FromSql for JsonObject<T> {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let conversion_res = serde_json::from_str(value.as_str()?);
        if let Ok(v) = conversion_res {
            Ok(Self::new(v))
        } else {
            Err(FromSqlError::InvalidType)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use rusqlite::Connection;
    use serde::{Deserialize, Serialize};

    #[test]
    fn insert_and_retrieve_bson_object() {
        let db = Connection::open_in_memory().expect("Failed to open connection");
        #[derive(Debug)]
        struct Foo {
            bar: BsonObject<Bar>,
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
        struct Bar {
            a: i64,
        }
        let f = Foo {
            bar: BsonObject::new(Bar { a: 10 }),
        };
        db.execute("create table foo( bar blob ) strict", ())
            .expect("failed to create table");

        let res = db.execute("insert into foo(bar) values (?)", (&f.bar,));
        assert!(res.is_ok(), "Failed to insert BsonObject: {:?}", res);

        let res = db.query_row("select * from foo", (), |row| {
            let bar: BsonObject<Bar> = row.get("bar")?;
            Ok(Foo { bar })
        });
        assert!(res.is_ok(), "Failed to retrieve BsonObject: {:?}", res);
        let value = res.unwrap();
        assert_eq!(value.bar.unwrap(), Bar { a: 10 });
    }

    #[test]
    fn insert_and_retrieve_json_object() {
        let db = Connection::open_in_memory().expect("Failed to open connection");
        #[derive(Debug)]
        struct Foo {
            bar: JsonObject<Bar>,
        }
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
        struct Bar {
            a: i64,
        }
        let f = Foo {
            bar: JsonObject::new(Bar { a: 10 }),
        };
        db.execute("create table foo( bar text ) strict", ())
            .expect("failed to create table");

        let res = db.execute("insert into foo(bar) values (?)", (&f.bar,));
        assert!(res.is_ok(), "Failed to insert JsonObject: {:?}", res);

        let res = db.query_row("select * from foo", (), |row| {
            let bar: JsonObject<Bar> = row.get("bar")?;
            Ok(Foo { bar })
        });
        assert!(res.is_ok(), "Failed to retrieve JsonObject: {:?}", res);
        let value = res.unwrap();
        assert_eq!(value.bar.unwrap(), Bar { a: 10 });
    }
}
