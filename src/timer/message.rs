#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum Message {
    Start,
    Pause,
    Reset,
    Second,
    Seconds(u64),
    ChangeTimerTime(u64),
}