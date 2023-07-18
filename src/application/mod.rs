use std::error::Error;

use crate::config::Config;
use crate::monitor::{self, Monitor};
use crate::terminal;

fn start_monitors(config: Config) -> Vec<Monitor> {
    let mut monitors = vec![];

    for (name, address) in config.addresses {
        let config = monitor::Config {
            address,
            timeout: config.timeout,
            interval: config.interval,
        };

        let monitor = Monitor::new(name, config);

        monitor.start();

        monitors.push(monitor);
    }

    monitors
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let monitors = start_monitors(config);

    let context = terminal::Context::new();

    let mut terminal = terminal::initialize()?;

    terminal::run(&mut terminal, &context, &monitors)?;

    terminal::destroy(&mut terminal)?;

    Ok(())
}
