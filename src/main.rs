#![feature(process_exitcode_placeholder)]

extern crate serenity;
extern crate directories;
extern crate config;
extern crate failure;

use failure::Error;
use serenity::prelude::*;

mod conf;

struct Handler {
    conf: conf::Config
}

impl Handler {
    pub fn new(conf: conf::Config) -> Handler {
        Handler {conf}
    }
}

impl EventHandler for Handler {

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
    println!("Client starting");
    client.start().map_err(|e| ::failure::err_msg(format!("{}", e)))
}
