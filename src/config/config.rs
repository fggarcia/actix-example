use crate::util::constants;

use hocon::HoconLoader;
use serde::Deserialize;
use std::result::Result;

#[derive(Deserialize, Clone, Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
    pub keep_alive: usize,
    pub payload_size: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct LogConfig {
    pub log_level: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ScyllaConfig {
    pub keyspace: String,
    pub user: String,
    pub password: String,
    pub hosts: Vec<String>,
    pub port: i16,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub server: ServerConfig,
    pub log_config: LogConfig,
    pub scylla_config: ScyllaConfig,
}

impl Config {
    pub fn get_config(env_opt: Option<String>) -> Result<Config, hocon::Error> {
        let env: String = env_opt.unwrap_or_else(|| String::from(constants::DEV));

        println!("Use environment: {}", env);

        let env_file = format!("conf/environments/{}/application.conf", env);
        let app_config_file = "conf/application.conf";

        let home_dir = Config::get_home_dir();
        let sensitive = format!("{}/sensitive.conf", home_dir);

        let initial_loader = HoconLoader::new();
        let loader = load_from(initial_loader, app_config_file);
        let loader = load_from(loader, env_file.as_str());
        let config: Config = load_from(loader, sensitive.as_str()).resolve()?;

        Ok(config)
    }

    fn get_home_dir() -> String {
        let home_config = r#"{system.home: ${HOME}}"#;
        let config = HoconLoader::new()
            .load_str(home_config)
            .unwrap()
            .hocon()
            .unwrap();
        config["system"]["home"].as_string().unwrap()
    }
}

fn load_from(hocon_loader: HoconLoader, file_name: &str) -> HoconLoader {
    println!("trying merging....file: {}", file_name);

    let merge = hocon_loader.load_file(&file_name);

    match merge {
        Ok(merge_result) => merge_result,
        Err(err) => {
            println!(
                "Error merging {:?} file cause {:?}",
                &file_name,
                err.to_string()
            );
            hocon_loader
        }
    }
}
