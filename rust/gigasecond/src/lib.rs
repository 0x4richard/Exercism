use time::Duration;
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let billion_seconds = Duration::seconds(1_000_000_000);
    start + billion_seconds
}
