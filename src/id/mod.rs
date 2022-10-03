use rusqlite::{types::FromSql, Row, ToSql};

pub mod integer;
pub use integer::IntegerId;

/// Reccomended set of traits for a primary key column
pub trait Id<'stmt>: TryFrom<&'stmt Row<'stmt>> + FromSql + ToSql {}
