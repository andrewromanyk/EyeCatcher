use crate::timer::{self, message};
use iced::{Background, Border, Color, Element, Length::Fill, Subscription, Task, widget::{self, Container, button, container::Style, row, text}, window};
use tray_icon::TrayIconEvent;

const DEFAULT_WORK_TIMER: u64 = 20*60;
const DEFAULT_RELAX_TIMER: u64 = 1*20;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum MainMessage {
    TimerMessage(u64, timer::message::Message),
    TimerSwitchScreen,
    SwitchScreen(Screen),
    TrayEvent(TrayIconEvent),
    CloseEvent
}

#[derive(Debug, Clone, Copy)]
pub enum Screen {
    Main,
    Relax,
}

#[derive(Debug, Clone, Copy)]
pub struct State {
    screen: Screen,
    timer_main: timer::Timer,
    timer_relax: timer::Timer,
}

impl State {
    pub fn update(&mut self, message: MainMessage) -> Task<MainMessage> {
        match message {
            MainMessage::TimerMessage(id, message) => match &id {
                1 => return self.timer_main.update(message),
                2 => return self.timer_relax.update(message),
                _ => panic!("Trying to modify non-existent timer")
            },
            MainMessage::TimerSwitchScreen => match &self.screen {
                Screen::Main => self.screen = Screen::Relax,
                _ => self.screen = Screen::Main
            }
            MainMessage::SwitchScreen(screen) => self.screen = screen,
            MainMessage::TrayEvent(a) => {
                match &a {
                    TrayIconEvent::DoubleClick { id: _, position: _, rect: _, button: _ } => {
                        return window::latest().and_then(|id| window::set_mode(id, window::Mode::Windowed));
                    }
                    _ => {}
                } 
            }
            MainMessage::CloseEvent => {
                return window::latest().and_then(|id| window::set_mode(id, window::Mode::Hidden));
            }
            // TODO: finish all messages
            _ => println!("Did not match all events do it RN")
        };
        Task::none()
    }

    pub fn view(&self) -> Element<'_, MainMessage> {
        widget::container(
            match &self.screen {
                Screen::Main => {
                    widget::column![
                        title_bar(),
                        text("Main Timer"),
                        self.timer_main.view(),
                        button("Play").on_press(MainMessage::TimerMessage(1, message::Message::Start)),
                        button("Pause").on_press(MainMessage::TimerMessage(1, message::Message::Pause))
                    ]
                },
                Screen::Relax => {
                    widget::column![
                        text("Relax timer"),
                        self.timer_relax.view(),
                        button("Play").on_press(MainMessage::TimerMessage(2, message::Message::Start)),
                        button("Pause").on_press(MainMessage::TimerMessage(2, message::Message::Pause))
                    ]
                }
            }
        )
        .width(Fill)
        .height(Fill)
        .style(|_| {
            Style {
                background: Some(Background::Color(Color::from_rgba8(25, 25, 25, 0.5))),
                border: Border { 
                    color: Color::WHITE, 
                    width: 1.5, 
                    radius: 12.into() },
                ..Default::default()
            }
        })
        .into()
    }

    pub fn subscription(&self) -> Subscription<MainMessage> {
        Subscription::batch([
            self.timer_main.subscription().map(|a| MainMessage::TimerMessage(1, a)),
            self.timer_relax.subscription().map(|a| MainMessage::TimerMessage(2, a))
        ])
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            screen: Screen::Main,
            timer_main: timer::Timer::quick_init(DEFAULT_WORK_TIMER),
            timer_relax: timer::Timer::quick_init(DEFAULT_RELAX_TIMER),
        }
    }
}

pub fn title_bar() -> Container<'static, MainMessage> {
    widget::container(
        row![
            button(text!("close").size(14))
        ]
        .width(Fill)
        .height(25)
    )
    .style(|_| {
        Style { 
            text_color: Some(Color::from_rgb8(31, 31, 31)), 
            background: Some(iced::Background::Color(Color::from_rgb8(24, 24, 24))), 
            ..Default::default() 
        }
    })
}