use std::time::Duration;

const TARGET_FPS: u64 = 10;

pub const TICK_RATE: Duration = Duration::from_millis(1000 / TARGET_FPS);
