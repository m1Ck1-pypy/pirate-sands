use crate::notifications::{notify_sound, show_notification};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerState {
    Running,
    Stopped,
    Paused,
}

#[derive(Debug)]
pub struct Timer {
    pub state: TimerState,
    pub remaining: Duration,
    pub start_time: u64,
    pub sound_notify: bool,
}

impl Timer {
    pub fn new(minutes: u64) -> Self {
        Self {
            remaining: Duration::from_secs(minutes * 60),
            state: TimerState::Stopped,
            start_time: minutes,
            sound_notify: true,
        }
    }

    pub fn update_minutes(&mut self, minutes: u64) {
        self.remaining = Duration::from_secs(minutes * 60);
        self.start_time = minutes;
    }

    pub fn set_one_minute(&mut self) {
        self.remaining += Duration::from_secs(60);
        self.start_time += 1;
    }

    pub fn reset(&mut self, minutes: u64) {
        self.remaining = Duration::from_secs(minutes * 60);
        self.state = TimerState::Stopped;
    }

    pub fn tick(&mut self) {
        if self.state == TimerState::Running && self.remaining.as_secs() > 0 {
            self.remaining -= Duration::from_secs(1);
        }

        if self.remaining.as_secs() == 0 {
            self.stop()
        }
    }

    pub fn stop(&mut self) {
        self.state = TimerState::Stopped;
        self.remaining = Duration::from_secs(self.start_time * 60);
        show_notification();
        if self.sound_notify {
            notify_sound();
        }
    }

    pub fn toggle(&mut self) {
        self.state = match self.state {
            TimerState::Running => TimerState::Paused,
            TimerState::Paused => TimerState::Running,
            TimerState::Stopped => TimerState::Running,
        };
    }
}

pub type TimerStateType = Arc<Mutex<Timer>>;
