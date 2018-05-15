use failure::Error;
use serenity::model::id::{GuildId, ChannelId};

use std::str::FromStr;

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Config {
    pub discord_token: String,
    pub guild_id: GuildId,
    pub channel_id: ChannelId,
    pub text_channel_id: ChannelId,
    pub audio_path: String
}

impl Config {
    pub fn from_conf(conf: &::config::Config) -> Result<Config, Error> {
        Ok(Config {
            discord_token: conf.get_str("discord_token").map_err(Error::from)?,
            guild_id: conf.get_str("guild_id")
                .map_err(Error::from)
                .and_then(|id| u64::from_str(&id).map_err(Error::from))
                .map(GuildId)?,
            channel_id: conf.get_str("channel_id")
                .map_err(Error::from)
                .and_then(|id| u64::from_str(&id).map_err(Error::from))
                .map(ChannelId)?,
            text_channel_id: conf.get_str("text_channel_id")
                .map_err(Error::from)
                .and_then(|id| u64::from_str(&id).map_err(Error::from))
                .map(ChannelId)?,
            audio_path: conf.get_str("audio_path").map_err(Error::from)?
        })
    }
}