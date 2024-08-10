#[macro_export]
macro_rules! tokio_sleep_ms {
    ($n: expr) => {{
        tokio::time::sleep(std::time::Duration::from_millis($n)).await;
    }};
}

#[macro_export]
macro_rules! db_string {
    ($n: expr) => {{
        format!("{:#x}", $n)
    }};
}

pub fn split_range(start: u64, end: u64, step: usize) -> impl Iterator<Item = (u64, u64)> {
    (start..end)
        .step_by(step)
        .map(move |current| (current, std::cmp::min(end, current + step as u64)))
}
