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
        Self(self.0.clone(), PhantomData)
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
