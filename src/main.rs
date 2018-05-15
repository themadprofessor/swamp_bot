#![feature(process_exitcode_placeholder)]

extern crate serenity;
extern crate directories;
extern crate config;
extern crate failure;
extern crate typemap;

use failure::Error;
use serenity::prelude::*;
use serenity::client::bridge::voice::ClientVoiceManager;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::voice::VoiceState;
use typemap::Key;

use std::sync::Arc;

mod conf;

struct VoiceManager;

struct Handler {
    conf: conf::Config
}

impl Handler {
    pub fn new(conf: conf::Config) -> Handler {
        Handler {conf}
    }
}

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, ready: Ready) {
        let data_lock = ctx.data.lock();
        let manager_lock = data_lock.get::<VoiceManager>().cloned().unwrap();
        let mut manager = manager_lock.lock();
        manager.join(self.conf.guild_id, self.conf.channel_id);
        println!("Joined");
    }

    fn voice_state_update(&self, ctx: Context, guild: Option<GuildId>, state: VoiceState) {
        let manager_lock = ctx.data.lock().get::<VoiceManager>().cloned().unwrap();
        let mut manager = manager_lock.lock();
        if let Some(handler) = manager.get_mut(self.conf.guild_id) {
            handler.play(::serenity::voice::ffmpeg(&self.conf.audio_path).unwrap());
            println!("Playing");
        }
    }
}



impl Key for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

fn main() -> std::process::ExitCode {
    match run() {
        Ok(_) => ::std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{}", e);
            ::std::process::ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Error> {
    let conf = {
        let project_dirs = ::directories::ProjectDirs::from("io", "discord", "swamp_bot");
        let mut config = ::config::Config::new();
        let paths = [project_dirs.data_dir(),
            project_dirs.data_local_dir(),
            project_dirs.config_dir()];

        for &path in &paths {
            if path.exists() {
                config.merge(config::File::with_name(path.to_str().unwrap()))?;
            }
        }
        config.merge(::config::Environment::with_prefix("swamp"))?;
        #[cfg(debug_assertions)] config.merge(::config::File::with_name("debug.toml"))?; //Only include debug.toml     if this a test scenario
        conf::Config::from_conf(&config)
    }?;
    println!("Config loaded");

    let mut client = Client::new(
        &conf.discord_token.clone(),
        Handler::new(conf))
        .map_err(|e| ::failure::err_msg(format!("{}", e)))?;
    {
        let mut data = client.data.lock();
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
    }
    println!("Client starting");
    client.start().map_err(|e| ::failure::err_msg(format!("{}", e)))
}
