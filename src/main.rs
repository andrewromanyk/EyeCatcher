mod timer;

use iced;

fn main() -> iced::Result {
    iced::application(timer::Timer::default, timer::Timer::update, timer::Timer::view)
        .subscription(timer::Timer::subscription)
        .run()
}