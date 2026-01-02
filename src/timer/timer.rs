mod Timer {
    
    #[derive(Debug, Clone, Copy)]
    struct Timer {
        timer_time: u64,
        remaining_time: u64,
        is_ticking: bool,
    }

    impl Timer {
        fn new(timer_time: u64, remaining_time: u64, is_ticking: bool) -> Self {
            Timer {
                timer_time,
                remaining_time,
                is_ticking
            }
        }
    }

    impl Default for Timer {
        fn default() -> Self {
            Timer {
                timer_time: 20*60,
                remaining_time: 20*60,
                is_ticking: false
            }
        }
    }
}