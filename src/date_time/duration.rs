use std::marker::PhantomData;

use rusqlite::{
    types::{FromSql, FromSqlError, ToSqlOutput},
    ToSql,
};
use thiserror::Error;

use super::{Microseconds, Milliseconds, Nanoseconds, Seconds};

pub type DurationSeconds = Duration<Seconds>;
pub type DurationMillis = Duration<Milliseconds>;
pub type DurationMicros = Duration<Microseconds>;
pub type DurationNanos = Duration<Nanoseconds>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration<Scale>(chrono::Duration, PhantomData<Scale>);
impl<Scale> Duration<Scale> {
    pub fn unwrap(self) -> chrono::Duration {
        self.0
    }
}
impl<Scale> From<chrono::Duration> for Duration<Scale> {
    fn from(v: chrono::Duration) -> Self {
        Self(v, PhantomData)
    }
}
impl<Scale> TryFrom<std::time::Duration> for Duration<Scale> {
    type Error = time::OutOfRangeError;

    fn try_from(v: std::time::Duration) -> Result<Self, Self::Error> {
        Ok(Self(chrono::Duration::from_std(v)?, PhantomData))
    }

}
impl FromSql for Duration<Seconds> {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let db_seconds = value.as_i64()?;
        if let Ok(duration) =
            chrono::Duration::from_std(std::time::Duration::from_secs(db_seconds.abs() as u64))
        {
            if db_seconds >= 0 {
                Ok(Self(duration, PhantomData))
            } else {
                Ok(Self(-duration, PhantomData))
            }
        } else {
            Err(FromSqlError::InvalidType)
        }
    }
}
impl ToSql for Duration<Seconds> {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.0.num_seconds()))
    }
}

impl FromSql for Duration<Milliseconds> {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let db_seconds = value.as_i64()?;
        if let Ok(duration) =
            chrono::Duration::from_std(std::time::Duration::from_millis(db_seconds.abs() as u64))
        {
            if db_seconds >= 0 {
                Ok(Self(duration, PhantomData))
            } else {
                Ok(Self(-duration, PhantomData))
            }
        } else {
            Err(FromSqlError::InvalidType)
        }
    }
}
impl ToSql for Duration<Milliseconds> {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.0.num_milliseconds()))
    }
}

impl FromSql for Duration<Microseconds> {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let db_seconds = value.as_i64()?;
        if let Ok(duration) =
            chrono::Duration::from_std(std::time::Duration::from_micros(db_seconds.abs() as u64))
        {
            if db_seconds >= 0 {
                Ok(Self(duration, PhantomData))
            } else {
                Ok(Self(-duration, PhantomData))
            }
        } else {
            Err(FromSqlError::InvalidType)
        }
    }
}
impl ToSql for Duration<Microseconds> {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        if let Some(us) = self.0.num_microseconds() {
            Ok(ToSqlOutput::from(us))
        } else {
            Err(rusqlite::Error::ToSqlConversionFailure(Box::new(
                Error::Overflow,
            )))
        }
    }
}

impl FromSql for Duration<Nanoseconds> {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let db_seconds = value.as_i64()?;
        if let Ok(duration) =
            chrono::Duration::from_std(std::time::Duration::from_nanos(db_seconds.abs() as u64))
        {
            if db_seconds >= 0 {
                Ok(Self(duration, PhantomData))
            } else {
                Ok(Self(-duration, PhantomData))
            }
        } else {
            Err(FromSqlError::InvalidType)
        }
    }
}
impl ToSql for Duration<Nanoseconds> {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        if let Some(ns) = self.0.num_nanoseconds() {
            Ok(ToSqlOutput::from(ns))
        } else {
            Err(rusqlite::Error::ToSqlConversionFailure(Box::new(
                Error::Overflow,
            )))
        }
    }
}

#[derive(Clone, Copy, Error, Debug)]
pub enum Error {
    #[error("Overflow")]
    Overflow,
}

#[cfg(test)]
mod test {
    use rusqlite::Connection;

    use super::*;

    #[test]
    fn insert_duration_s_and_retrieve() {
        let db = Connection::open_in_memory().expect("Failed to open connection");

        db.execute("create table foo( a integer ) strict", ())
            .expect("failed to create table");
        let stored_duration =
            chrono::Duration::from_std(std::time::Duration::from_secs(5)).unwrap();
        let res = db.query_row(
            "insert into foo(a) values(?) returning *",
            (DurationSeconds::from(stored_duration),),
            |row| {
                let v: DurationSeconds = row.get("a")?;
                Ok(v)
            },
        );
        assert!(
            res.is_ok(),
            "Failed to retrieve timestamp from database: {:?}",
            res
        );
        let retrieved_duration = res.unwrap().unwrap();
        assert_eq!(
            stored_duration.num_seconds(),
            retrieved_duration.num_seconds(),
            "Stored duration does not equal retrieved duration"
        );
    }

    #[test]
    fn insert_duration_ms_and_retrieve() {
        let db = Connection::open_in_memory().expect("Failed to open connection");

        db.execute("create table foo( a integer ) strict", ())
            .expect("failed to create table");
        let stored_duration =
            chrono::Duration::from_std(std::time::Duration::from_millis(5)).unwrap();
        let res = db.query_row(
            "insert into foo(a) values(?) returning *",
            (DurationMillis::from(stored_duration),),
            |row| {
                let v: DurationMillis = row.get("a")?;
                Ok(v)
            },
        );
        assert!(
            res.is_ok(),
            "Failed to retrieve timestamp from database: {:?}",
            res
        );
        let retrieved_duration = res.unwrap().unwrap();
        assert_eq!(
            stored_duration.num_milliseconds(),
            retrieved_duration.num_milliseconds(),
            "Stored duration does not equal retrieved duration"
        );
    }

    #[test]
    fn insert_duration_us_and_retrieve() {
        let db = Connection::open_in_memory().expect("Failed to open connection");

        db.execute("create table foo( a integer ) strict", ())
            .expect("failed to create table");
        let stored_duration =
            chrono::Duration::from_std(std::time::Duration::from_micros(5)).unwrap();
        let res = db.query_row(
            "insert into foo(a) values(?) returning *",
            (DurationMicros::from(stored_duration),),
            |row| {
                let v: DurationMicros = row.get("a")?;
                Ok(v)
            },
        );
        assert!(
            res.is_ok(),
            "Failed to retrieve timestamp from database: {:?}",
            res
        );
        let retrieved_duration = res.unwrap().unwrap();
        assert_eq!(
            stored_duration.num_microseconds().unwrap(),
            retrieved_duration.num_microseconds().unwrap(),
            "Stored duration does not equal retrieved duration"
        );
    }

    #[test]
    fn insert_duration_ns_and_retrieve() {
        let db = Connection::open_in_memory().expect("Failed to open connection");

        db.execute("create table foo( a integer ) strict", ())
            .expect("failed to create table");
        let stored_duration =
            chrono::Duration::from_std(std::time::Duration::from_nanos(5)).unwrap();
        let res = db.query_row(
            "insert into foo(a) values(?) returning *",
            (DurationNanos::from(stored_duration),),
            |row| {
                let v: DurationNanos = row.get("a")?;
                Ok(v)
            },
        );
        assert!(
            res.is_ok(),
            "Failed to retrieve timestamp from database: {:?}",
            res
        );
        let retrieved_duration = res.unwrap().unwrap();
        assert_eq!(
            stored_duration.num_nanoseconds().unwrap(),
            retrieved_duration.num_nanoseconds().unwrap(),
            "Stored duration does not equal retrieved duration"
        );
    }
}
