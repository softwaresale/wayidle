use std::{fs::File, io::BufReader, path::PathBuf, time::Duration};

use serde::{Deserialize, Deserializer};

use crate::err::{WayIdleError, WayIdleResult};

#[derive(Deserialize, Debug)]
pub struct WayIdleConfig {
    pub(crate) idle_config: IdleConfig,
}

impl WayIdleConfig {
    pub fn load(config_file_path: Option<PathBuf>) -> WayIdleResult<Self> {
        let config_path = config_file_path
            .or(Self::default_config_file_path())
            .ok_or(WayIdleError::ConfigFileMissing)?;

        let config_file = File::open(config_path)?;

        let config_contents = std::io::read_to_string(BufReader::new(config_file))?;

        let config = toml::from_str::<WayIdleConfig>(&config_contents)?;

        Ok(config)
    }

    fn default_config_file_path() -> Option<PathBuf> {
        let config_dir = std::env::var("XDG_CONFIG_DIR").ok()?;
        let config_file_path = PathBuf::from(config_dir).join("waylock/config.toml");

        if !config_file_path.exists() {
            return None;
        }

        Some(config_file_path)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct IdleConfig {
    /// how long to wait until idle
    #[serde(deserialize_with = "serde_parse_duration")]
    duration: Duration,
    /// the command to run once idle state is detected
    command: Vec<String>,
}

impl IdleConfig {
    pub fn duration(&self) -> Duration {
        self.duration
    }

    pub fn command(&self) -> &[String] {
        self.command.as_slice()
    }
}

fn serde_parse_duration<'de, D>(deserialize: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let duration_string = String::deserialize(deserialize)?;
    parse_duration::parse(&duration_string).map_err(|err| serde::de::Error::custom(err.to_string()))
}
