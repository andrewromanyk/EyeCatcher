mod state;
mod timer;
mod tray;

use iced::{self, Subscription, window::{self, Settings}};

use crate::tray::tray::tray_icon;

fn main() -> iced::Result {
    let icon_raw = include_bytes!("images/icon.jpg");
    
    let _tray_icon = tray_icon(icon_raw);

    iced::application(
        state::State::default,
        state::State::update,
        state::State::view,
    )
    .window(Settings {
        icon: Some(window::icon::from_file_data(icon_raw, Option::None).unwrap()),
        ..Default::default()
    })
    .subscription(|state| Subscription::batch([
        state::State::subscription(state),
        tray::tray::tray_subscription()
    ]))
    .run()
}
