use config::ConfigError;

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Config {
    pub discord_token: String,
    pub cmd_char: String,
}

impl Config {
    pub fn from_conf(conf: &::config::Config) -> Result<Config, ConfigError> {
        Ok(Config {
            discord_token: conf.get_str("discord_token")?,
            cmd_char: conf.get_str("cmd_char")?,
        })
    }
}