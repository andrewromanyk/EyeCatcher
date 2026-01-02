#[derive(Clone, Copy, Debug)]
pub enum Message {
    Start,
    Pause,
    Reset,
    Second,
    Seconds(u64),
    ChangeTimerTime(u64),
}