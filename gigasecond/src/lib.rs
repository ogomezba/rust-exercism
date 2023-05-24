use std::time::Duration;
use time::PrimitiveDateTime as DateTime;

static GIGASECONDS: u64 = 1000000000000;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let duration = Duration::from_millis(GIGASECONDS);
    start + duration
}
