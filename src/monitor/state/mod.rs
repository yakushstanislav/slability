use std::time::Instant;
use time::OffsetDateTime;

#[derive(Clone, Copy)]
pub struct State {
    is_online: Option<bool>,

    last_online: Option<OffsetDateTime>,
    elapsed_online: Option<Instant>,

    last_offline: Option<OffsetDateTime>,
    elapsed_offline: Option<Instant>,

    restarts: u32,
}

impl State {
    pub fn new() -> Self {
        State {
            is_online: None,

            last_online: None,
            elapsed_online: None,

            last_offline: None,
            elapsed_offline: None,

            restarts: 0,
        }
    }

    pub fn set_is_online(&mut self, is_online: bool) {
        if let Some(true) = self.is_online {
            if !is_online {
                self.last_online = None;
                self.elapsed_online = None;

                self.last_offline = Some(OffsetDateTime::now_local().unwrap());
                self.elapsed_offline = Some(Instant::now());
            }
        } else if let Some(false) = self.is_online {
            if is_online {
                self.last_online = Some(OffsetDateTime::now_local().unwrap());
                self.elapsed_online = Some(Instant::now());

                self.last_offline = None;
                self.elapsed_offline = None;

                self.restarts = self.restarts + 1;
            }
        }

        self.is_online = Some(is_online)
    }

    pub fn is_online(&self) -> Option<bool> {
        self.is_online
    }

    pub fn last_online(&self) -> Option<OffsetDateTime> {
        self.last_online
    }

    pub fn elapsed_online(&self) -> Option<Instant> {
        self.elapsed_online
    }

    pub fn last_offline(&self) -> Option<OffsetDateTime> {
        self.last_offline
    }

    pub fn elapsed_offline(&self) -> Option<Instant> {
        self.elapsed_offline
    }

    pub fn restarts(&self) -> u32 {
        self.restarts
    }
}
