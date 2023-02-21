use anyhow::Result;
use gh_config::{Config as GhCliConfig, Host};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

const APP_CONFIG_DIR: &str = "ghcli-tui";
const CONFIG_FILE: &str = "config.yml";

#[derive(Debug, Clone)]
pub struct GhConfig {
    ghcli_config: GhCliConfig,
    ghcli_host_config: Host,
}

pub fn read_user_config() -> Result<String> {
    let config_dir = dirs::config_dir().expect("unable to find system config dir");
    let config_path = config_dir.join(format!(
        "/{app_config_dir}/{config_file}",
        app_config_dir = APP_CONFIG_DIR,
        config_file = CONFIG_FILE
    ));

    Ok(fs::read_to_string(config_path)?)
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UserConfig {}
