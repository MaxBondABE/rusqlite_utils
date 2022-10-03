use std::marker::PhantomData;

use chrono::NaiveDateTime;
use rusqlite::{
    types::{FromSql, ToSqlOutput},
    ToSql,
};
use serde::{Deserialize, Serialize};

pub type UnixEpoch = Timestamp<Seconds>;
pub type TimestampMillis = Timestamp<Milliseconds>;
pub type TimestampNanos = Timestamp<Nanoseconds>;

type _UtcDateTime = chrono::DateTime<chrono::Utc>;

/// Stores a timestamp as a SQLite INTEGER. The type is used to specify the
/// scale at which to store the timestamp, eg, a Timstamp<Second> will store
/// an integer number of seconds in it's column, and at Timestamp<Milliseconds>
/// will store that number in Milliseconds.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Timestamp<Scale>(_UtcDateTime, PhantomData<Scale>);
impl<T> Timestamp<T> {
    pub fn now() -> Self {
        chrono::Utc::now().into()
    }
}
impl<T> From<_UtcDateTime> for Timestamp<T> {
    fn from(v: chrono::DateTime<chrono::Utc>) -> Self {
        Self(v, PhantomData)
    }
}
impl<T> Into<_UtcDateTime> for Timestamp<T> {
    fn into(self) -> _UtcDateTime {
        self.0
    }
}

/// Record timestamps at the second scale.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Seconds {}

impl FromSql for Timestamp<Seconds> {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let v = value.as_i64()?;
        let timestamp = NaiveDateTime::from_timestamp(v, 0);
        Ok(_UtcDateTime::from_utc(timestamp, chrono::Utc).into())
    }
}
impl ToSql for Timestamp<Seconds> {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.0.timestamp()))
    }
}

/// Record timestamps at the millisecond scale.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Milliseconds {}

impl FromSql for Timestamp<Milliseconds> {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        const MILLI_PER_SECOND: i64 = 1000;
        const NANO_PER_MILLI: i64 = 1000;

        let v = value.as_i64()?;
        let v_secs = v.div_euclid(MILLI_PER_SECOND);
        let v_nanos = (v.rem_euclid(NANO_PER_MILLI) * NANO_PER_MILLI) as u32;
        // Because v_nanos is at most 999000, we can safely cast down to u32

        let timestamp = NaiveDateTime::from_timestamp(v_secs, v_nanos);
        Ok(_UtcDateTime::from_utc(timestamp, chrono::Utc).into())
    }
}
impl ToSql for Timestamp<Milliseconds> {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.0.timestamp_millis()))
    }
}

/// Record timestamps at the nanosecond scale.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Nanoseconds {}

impl FromSql for Timestamp<Nanoseconds> {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        const NANO_PER_SECOND: i64 = 1_000_000;

        let v = value.as_i64()?;
        let v_secs = v.div_euclid(NANO_PER_SECOND);
        let v_nanos = v.rem_euclid(NANO_PER_SECOND) as u32;
        // Because v_nanos is at most 999999, we can safely cast down to u32

        let timestamp = NaiveDateTime::from_timestamp(v_secs, v_nanos);
        Ok(_UtcDateTime::from_utc(timestamp, chrono::Utc).into())
    }
}
impl ToSql for Timestamp<Nanoseconds> {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.0.timestamp_nanos()))
    }
}
