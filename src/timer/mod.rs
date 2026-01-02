pub mod message;

use std::time;
use iced::widget::{Text, text};
use iced::{Subscription, Task};
use message::Message::{self, *};
use crate::state::MainMessage;
use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct Timer {
    pub timer_time: u64,
    pub remaining_time: u64,
    pub is_ticking: bool,
}

impl Timer {
    pub fn quick_init(timer_time: u64) -> Self {
        Timer {
            timer_time,
            remaining_time: timer_time,
            is_ticking: false,
        }
    }

    #[allow(dead_code)]
    pub fn new(timer_time: u64, remaining_time: u64, is_ticking: bool) -> Self {
        Timer {
            timer_time,
            remaining_time,
            is_ticking,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<MainMessage>  {
        match &message {
            Start => self.is_ticking = true,
            Pause => self.is_ticking = false,
            Reset => {
                self.is_ticking = false;
                self.remaining_time = self.timer_time
            }
            ChangeTimerTime(time) => self.timer_time = *time,
            Second => {
                if let Some(_) = self.reduce(1) {
                    return Task::done(MainMessage::TimerSwitchScreen);
                }
            },
            Seconds(seconds) => {
                if let Some(_) = self.reduce(*seconds) {
                    return Task::done(MainMessage::TimerSwitchScreen);
                }
            }
        }

        Task::none()
    }

    pub fn reduce(&mut self, seconds: u64) -> Option<()> {
        if self.remaining_time <= seconds {
            self.is_ticking = false;
            self.remaining_time = self.timer_time;
            return Some(());
        }
        self.remaining_time -= seconds;
        None
    }

    pub fn view(&self) -> Text<'_> {
        text!(
            "{:02}:{:02}",
            self.remaining_time / 60,
            self.remaining_time % 60
        )
    }

    pub fn subscription(&self) -> Subscription<Message> {
        if self.is_ticking {
            return iced::time::every(time::Duration::from_secs(1))
                .map(|_| Message::Second);
        }
        Subscription::none()
    }
}

impl Default for Timer {
    fn default() -> Self {
        Timer {
            timer_time: 20 * 60,
            remaining_time: 20 * 60,
            is_ticking: false,
        }
    }
}
