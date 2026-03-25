use std::{path::PathBuf, process::ExitCode};
use tracing::{Level, error};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::configuration::config::{Config, ConfigError, load_or_create};

pub async fn start_server(config_path: PathBuf, logging_level: u8) -> ExitCode {
    enable_logging(logging_level);
    let Some(cfg) = load_configuration(&config_path) else {
        return ExitCode::FAILURE;
    };

    ExitCode::SUCCESS
}

fn load_configuration(config_path: &PathBuf) -> Option<Config> {
    let cfg = load_or_create(config_path);
    match cfg {
        Err(ConfigError::TomlDeserialize(message, ..)) => {
            error!("Faile to load configuration: {}", message);
        }
        Err(ConfigError::Io(message, ..)) => {
            error!("Faile to load configuration: {}", message);
        }
        Err(ConfigError::TomlSerialize(message, ..)) => {
            error!("Faile to load default configuration: {}", message);
        }
        Err(ConfigError::EnvPlaceholder(var)) => {
            error!("Faile to load configuration: {}", var);
        }
        Ok(cfg) => return Some(cfg),
    }
    None
}

fn enable_logging(verbose: u8) {
    let log_level = match verbose {
        0 => Level::INFO,
        1 => Level::DEBUG,
        _ => Level::TRACE,
    };

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(log_level.into()))
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .init();
}
