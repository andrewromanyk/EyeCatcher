mod Message {

    #[derive(Clone, Copy, Debug)]
    enum Message {
        Start,
        Pause,
        Reset,
        Change_timer_time(u64),
    }

}