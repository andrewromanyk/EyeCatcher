pub mod message;

use iced::Alignment::Center;
use iced::Length::Fill;
use iced::widget::{button, text, column, Column};
use iced::Subscription;
use std::time;
use message::Message;
use message::Message::{*};


#[derive(Debug, Clone, Copy)]
pub struct Timer {
    pub timer_time: u64,
    pub remaining_time: u64,
    pub is_ticking: bool,
}

impl Timer {
    pub fn new(timer_time: u64, remaining_time: u64, is_ticking: bool) -> Self {
        Timer {
            timer_time,
            remaining_time,
            is_ticking
        }
    }

    pub fn update(&mut self, message: Message) {
        match &message {
            Start => {
                self.is_ticking = true
            },
            Pause => {
                self.is_ticking = false
            },
            Reset => {
                self.is_ticking = false;
                self.remaining_time = self.timer_time
            },
            ChangeTimerTime(time) => {
                self.timer_time = *time
            },
            Second => {
                self.remaining_time -= 1
            }
            Seconds(seconds) => {
                self.remaining_time -= *seconds
            }
        }
    }

    pub fn view(&self) -> Column<message::Message> {
        column![
            text!("{}:{:02}", self.remaining_time / 60, self.remaining_time % 60), 
            button("Play").on_press(message::Message::Start),
            button("Pause").on_press(message::Message::Pause)
        ]
        .width(Fill)
        .align_x(Center)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        if self.is_ticking {
            return iced::time::every(time::Duration::from_secs(1)).map(|_| Message::Second);
        }
        Subscription::none()
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