pub const OK: i32 = 0;
pub const WARNING: i32 = 1;
pub const CRITICAL: i32 = 2;
pub const UNKNOWN: i32 = 3;

pub const STATUS_PREFIX: [&str; 4] = ["OK", "WARNING", "CRITICAL", "UNKNOWN"];

pub struct NagiosState {
    pub status: i32,
    pub message: String,
}
