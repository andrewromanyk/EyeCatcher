use crate::timer::{self, message};
use iced::{Subscription, Task, widget::{self, button, text}};

#[derive(Debug, Clone, Copy)]
pub enum Screen {
    Main,
    Relax,
}

#[derive(Debug, Clone, Copy)]
pub enum MainMessage {
    TimerMessage(u64, timer::message::Message),
    TimerSwitchScreen,
    SwitchScreen(Screen)
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
            MainMessage::SwitchScreen(screen) => self.screen = screen
        };
        Task::none()
    }

    pub fn view(&self) -> widget::Column<'_, MainMessage> {
        match &self.screen {
            Screen::Main => {
                widget::column![
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
    }

    pub fn subscription(&self) -> Subscription<MainMessage> {
        Subscription::batch(vec![
            self.timer_main.subscription().map(|a| MainMessage::TimerMessage(1, a)),
            self.timer_relax.subscription().map(|a| MainMessage::TimerMessage(2, a))
        ])
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            screen: Screen::Main,
            timer_main: timer::Timer::quick_init(20),
            timer_relax: timer::Timer::quick_init(20),
        }
    }
}
