mod state;
mod timer;
mod tray;

use iced::{self, Color, Size, Subscription, theme::Style, window::{self, Settings}};

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
        visible: false,
        resizable: false,
        size: Size {width: 350.0, height: 200.0},
        decorations: false,
        exit_on_close_request: false,
        transparent: true,
        ..Default::default()
    })
    .subscription(|state| Subscription::batch([
        state.subscription(),
        tray::tray::tray_subscription(),
        window::close_requests().map(|_| state::MainMessage::CloseEvent)
    ]))
    .style(|_state, _theme| {
        Style {
            background_color: Color::from_rgba8(255, 255, 255, 1.0), 
            text_color: Color::WHITE,
        }
    })
    .run()
}
