mod state;
mod timer;

use iced::{self};

fn main() -> iced::Result {
    iced::application(
        state::State::default,
        state::State::update,
        state::State::view,
    )
    .subscription(
        state::State::subscription
    )
    .run()
}
