use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub mod state;

use state::State;

#[derive(Clone, Copy)]
pub struct Config {
    pub address: SocketAddr,
    pub timeout: Duration,
    pub interval: Duration,
}

pub struct Monitor {
    pub config: Config,

    state: Arc<Mutex<State>>,
}

impl Monitor {
    pub fn new(config: Config) -> Self {
        Monitor {
            config,
            state: Arc::new(Mutex::new(State::new())),
        }
    }

    pub fn start(&self) {
        let config = Arc::new(self.config);
        let state = Arc::clone(&self.state);

        std::thread::spawn(move || loop {
            let is_online = match TcpStream::connect_timeout(&config.address, config.timeout) {
                Ok(_) => true,
                Err(_) => false,
            };

            state.lock().unwrap().set_is_online(is_online);

            std::thread::sleep(config.interval);
        });
    }

    pub fn address(&self) -> SocketAddr {
        self.config.address
    }

    pub fn get_state(&self) -> State {
        self.state.lock().unwrap().clone()
    }
}
