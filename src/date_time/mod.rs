use serde::{Deserialize, Serialize};

pub mod duration;
pub mod timestamp;

pub use duration::{Duration, DurationMicros, DurationMillis, DurationNanos, DurationSeconds};
pub use timestamp::{TimestampMicros, TimestampMillis, TimestampNanos, UnixEpoch};

/// Record timestamps at the second scale.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Seconds {}

/// Record timestamps at the millisecond scale.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Milliseconds {}

/// Record timestamps at the millisecond scale.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Microseconds {}

/// Record timestamps at the nanosecond scale.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Nanoseconds {}
