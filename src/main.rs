#![allow(unused_imports, dead_code, unused_variables)]

mod cli;
mod config;
mod instance;
mod ui;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use config::{read_user_config, GhConfig, UserConfig};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use gh_config::{Config as GhCliConfig, Host, Hosts, GITHUB_COM};
use instance::Instance;
use octocrab::Octocrab;
use std::sync::Arc;

/// Attempt to retrieve the user's GitHub authentication token from the respected `GH_TOKEN` and `GITHUB_TOKEN` environment variables.
/// This allows the user to circumvent the need to run `gh auth login`.
fn retrieve_token_in_env() -> Option<&'static str> {
    option_env!("GH_TOKEN").xor(option_env!("GITHUB_TOKEN"))
}

/// Attempt to retrieve the GitHub CLI host config and general config. If it is the user's first time using the GitHub CLI when using this application,
/// this configuration will not be present in `SYSTEM_CONFIG_DIR/gh`. If this function fails, it will not prevent the UI from being loaded.
/// But, when the UI launches, the user will have to properly authenticate themselves.
fn retrieve_ghcli_config() -> Result<(GhCliConfig, Host)> {
    // TODO: add support for GitHub Enterprise Server hosts

    let ghcli_config = GhCliConfig::load()?;

    // it is ok to unwrap here because if we get to this point, the user has already authenticated themselves before
    let ghcli_host_config = Hosts::load()?.get(GITHUB_COM).cloned().unwrap();

    Ok((ghcli_config, ghcli_host_config))
}

async fn start_ui(instance: &Arc<Instance>, auth_prompt_on_start: bool) -> Result<()> {
    todo!();
}

#[tokio::main]
async fn main() -> Result<()> {
    let instance = {
        let mut gh_client_builder = Octocrab::builder();

        if let Some(token) = retrieve_token_in_env() {
            gh_client_builder = gh_client_builder.personal_token(token.into());
        } else if let Ok((ghcli_config, ghcli_host_config)) = retrieve_ghcli_config() {
            todo!();
        }

        let user_config = serde_yaml::from_str(&config::read_user_config()?)?;
        let gh_client = octocrab::initialise(gh_client_builder)?;

        Instance::new(user_config, gh_client)
    };

    Ok(())
}
