use std::marker::PhantomData;

use chrono::NaiveDateTime;
use rusqlite::{
    types::{FromSql, FromSqlError, ToSqlOutput},
    ToSql,
};
use serde::{Deserialize, Serialize};

use super::{Microseconds, Milliseconds, Nanoseconds, Seconds};

pub type UnixEpoch = Timestamp<Seconds>;
pub type TimestampMillis = Timestamp<Milliseconds>;
pub type TimestampMicros = Timestamp<Microseconds>;
pub type TimestampNanos = Timestamp<Nanoseconds>;

type _UtcDateTime = chrono::DateTime<chrono::Utc>;

/// Stores a timestamp as a SQLite INTEGER. The type is used to specify the
/// scale at which to store the timestamp, eg, a Timstamp<Second> will store
/// an integer number of seconds in it's column, and at Timestamp<Milliseconds>
/// will store that number in Milliseconds.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Timestamp<Scale>(_UtcDateTime, PhantomData<Scale>);
impl<T> Timestamp<T> {
    pub fn unwrap(self) -> _UtcDateTime {
        self.0
    }
    pub fn now() -> Self {
        chrono::Utc::now().into()
    }
}
impl<T> From<_UtcDateTime> for Timestamp<T> {
    fn from(v: chrono::DateTime<chrono::Utc>) -> Self {
        Self(v, PhantomData)
    }
}
impl<T> From<Timestamp<T>> for _UtcDateTime {
    fn from(v: Timestamp<T>) -> Self {
        v.0
    }
}

impl FromSql for Timestamp<Seconds> {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let db_seconds = value.as_i64()?;
        if let Some(timestamp) = NaiveDateTime::from_timestamp_opt(db_seconds, 0) {
            Ok(_UtcDateTime::from_utc(timestamp, chrono::Utc).into())
        } else {
            Err(FromSqlError::OutOfRange(db_seconds))
        }
    }
}
impl ToSql for Timestamp<Seconds> {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.0.timestamp()))
    }
}

impl FromSql for Timestamp<Milliseconds> {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        const MILLI_PER_SECOND: i64 = 1000;
        const NANO_PER_MILLI: i64 = 1_000_000;

        let db_millis = value.as_i64()?;
        let v_secs = db_millis.div_euclid(MILLI_PER_SECOND);
        let v_nanos = (db_millis.rem_euclid(MILLI_PER_SECOND) * NANO_PER_MILLI) as u32;
        // Because v_nanos is at most 999000, we can safely cast down to u32

        if let Some(timestamp) = NaiveDateTime::from_timestamp_opt(v_secs, v_nanos) {
            Ok(_UtcDateTime::from_utc(timestamp, chrono::Utc).into())
        } else {
            Err(FromSqlError::OutOfRange(db_millis))
        }
    }
}
impl ToSql for Timestamp<Milliseconds> {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.0.timestamp_millis()))
    }
}

impl FromSql for Timestamp<Microseconds> {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        const MICROS_PER_SECOND: i64 = 1_000_000;
        const NANO_PER_MICRO: i64 = 1_000;

        let db_micros = value.as_i64()?;
        let v_secs = db_micros.div_euclid(MICROS_PER_SECOND);
        let v_nanos = (db_micros.rem_euclid(MICROS_PER_SECOND) * NANO_PER_MICRO) as u32;
        // Because v_nanos is at most 999000, we can safely cast down to u32

        if let Some(timestamp) = NaiveDateTime::from_timestamp_opt(v_secs, v_nanos) {
            Ok(_UtcDateTime::from_utc(timestamp, chrono::Utc).into())
        } else {
            Err(FromSqlError::OutOfRange(db_micros))
        }
    }
}
impl ToSql for Timestamp<Microseconds> {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.0.timestamp_micros()))
    }
}

impl FromSql for Timestamp<Nanoseconds> {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        const NANO_PER_SECOND: i64 = 1_000_000_000;

        let db_nanos = value.as_i64()?;
        let v_secs = db_nanos.div_euclid(NANO_PER_SECOND);
        let v_nanos = db_nanos.rem_euclid(NANO_PER_SECOND) as u32;
        // Because v_nanos is at most 999999, we can safely cast down to u32

        if let Some(timestamp) = NaiveDateTime::from_timestamp_opt(v_secs, v_nanos) {
            Ok(_UtcDateTime::from_utc(timestamp, chrono::Utc).into())
        } else {
            Err(FromSqlError::OutOfRange(db_nanos))
        }
    }
}
impl ToSql for Timestamp<Nanoseconds> {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.0.timestamp_nanos()))
    }
}

#[cfg(test)]
mod test {
    use rusqlite::Connection;

    use super::*;

    #[test]
    fn retrieve_unixepoch_from_default() {
        let db = Connection::open_in_memory().expect("Failed to open connection");

        db.execute("create table foo( a integer default (unixepoch()) )", ())
            .expect("failed to create table");
        let res = db.query_row("insert into foo default values returning *", (), |row| {
            let v: UnixEpoch = row.get("a")?;
            Ok(v)
        });
        let rust_time = chrono::Utc::now();
        assert!(
            res.is_ok(),
            "Failed to retrieve timestamp from database: {:?}",
            res
        );
        let db_time: _UtcDateTime = res.unwrap().into();
        let delta = db_time - rust_time;
        assert!(
            delta.num_milliseconds().abs() < 1_000,
            "Timestamps are improbably far apart (DB: {:?} - Rust: {:?}).",
            db_time,
            rust_time
        );
    }

    #[test]
    fn retrieve_timestamp_ms_from_default() {
        let db = Connection::open_in_memory().expect("Failed to open connection");

        db.execute(
            "create table foo( a integer default (unixepoch() * 1000) )",
            (),
        )
        .expect("failed to create table");
        let res = db.query_row("insert into foo default values returning *", (), |row| {
            let v: TimestampMillis = row.get("a")?;
            Ok(v)
        });
        let rust_time = chrono::Utc::now();
        assert!(
            res.is_ok(),
            "Failed to retrieve timestamp from database: {:?}",
            res
        );
        let db_time: _UtcDateTime = res.unwrap().into();
        let delta = db_time - rust_time;
        assert!(
            delta.num_milliseconds().abs() < 1_000,
            "Timestamps are improbably far apart (DB: {:?} - Rust: {:?}).",
            db_time,
            rust_time
        );
    }

    #[test]
    fn retrieve_timestamp_ns_from_default() {
        let db = Connection::open_in_memory().expect("Failed to open connection");

        db.execute(
            "create table foo( a integer default (unixepoch() * 1000000000) )",
            (),
        )
        .expect("failed to create table");
        let res = db.query_row("insert into foo default values returning *", (), |row| {
            let v: TimestampNanos = row.get("a")?;
            Ok(v)
        });
        let rust_time = chrono::Utc::now();
        assert!(
            res.is_ok(),
            "Failed to retrieve timestamp from database: {:?}",
            res
        );
        let db_time: _UtcDateTime = res.unwrap().into();
        let delta = db_time - rust_time;
        assert!(
            delta.num_milliseconds().abs() < 1_000,
            "Timestamps are improbably far apart (DB: {:?} - Rust: {:?}).",
            db_time,
            rust_time
        );
    }

    #[test]
    fn insert_unixepoch_and_retrieve() {
        let db = Connection::open_in_memory().expect("Failed to open connection");

        db.execute("create table foo( a integer )", ())
            .expect("failed to create table");
        let stored_time = UnixEpoch::now();
        let res = db.query_row(
            "insert into foo(a) values(?) returning *",
            (stored_time,),
            |row| {
                let v: UnixEpoch = row.get("a")?;
                Ok(v)
            },
        );
        assert!(
            res.is_ok(),
            "Failed to retrieve timestamp from database: {:?}",
            res
        );
        let retrieved_time = res.unwrap();
        let st_dt: _UtcDateTime = stored_time.into();
        let rt_dt: _UtcDateTime = retrieved_time.into();
        assert_eq!(st_dt.timestamp(), rt_dt.timestamp());
    }

    #[test]
    fn insert_timestamp_ms_and_retrieve() {
        let db = Connection::open_in_memory().expect("Failed to open connection");

        db.execute("create table foo( a integer )", ())
            .expect("failed to create table");
        let stored_time = TimestampMillis::now();
        let res = db.query_row(
            "insert into foo(a) values(?) returning *",
            (stored_time,),
            |row| {
                let v: TimestampMillis = row.get("a")?;
                Ok(v)
            },
        );
        assert!(
            res.is_ok(),
            "Failed to retrieve timestamp from database: {:?}",
            res
        );
        let retrieved_time = res.unwrap();
        let st_dt: _UtcDateTime = stored_time.into();
        let rt_dt: _UtcDateTime = retrieved_time.into();
        assert_eq!(st_dt.timestamp_millis(), rt_dt.timestamp_millis());
    }

    #[test]
    fn insert_timestamp_ns_and_retrieve() {
        let db = Connection::open_in_memory().expect("Failed to open connection");

        db.execute("create table foo( a integer )", ())
            .expect("failed to create table");
        let stored_time = TimestampNanos::now();
        let res = db.query_row(
            "insert into foo(a) values(?) returning *",
            (stored_time,),
            |row| {
                let v: TimestampNanos = row.get("a")?;
                Ok(v)
            },
        );
        assert!(
            res.is_ok(),
            "Failed to retrieve timestamp from database: {:?}",
            res
        );
        let retrieved_time = res.unwrap();
        let st_dt: _UtcDateTime = stored_time.into();
        let rt_dt: _UtcDateTime = retrieved_time.into();
        assert_eq!(st_dt.timestamp_nanos(), rt_dt.timestamp_nanos());
    }
}
