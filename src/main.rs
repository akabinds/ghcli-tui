#![allow(dead_code, unused_variables, unused_mut)]

mod cli;
mod instance;
mod ui;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use gh_config::{Config as GhCliConfig, Host as GhCliHostConfig, Hosts, GITHUB_COM};
use instance::Instance;
use octocrab::Octocrab;
use std::{io, sync::Arc};
use tokio::sync::Mutex as TokioMutex;
use tui::{backend::CrosstermBackend, Terminal};

/// Attempt to retrieve the user's personal authentication token for the `github.com` host from the respected `GH_TOKEN` and `GITHUB_TOKEN` environment variables.
/// This allows the user to circumvent the need to run `gh auth login`.
fn retrieve_ghcom_pat_from_env() -> Option<&'static str> {
    option_env!("GH_TOKEN").xor(option_env!("GITHUB_TOKEN"))
}

/// Attempt to retrieve the GitHub CLI general and host config. These configuration files will be present, in either
/// `$XDG_CONFIG_HOME/gh` or the directory specified by the `GH_CONFIG_DIR` environment variable, if the user has
/// already run `gh auth login`.
fn retrieve_ghcli_config() -> Result<(GhCliConfig, GhCliHostConfig)> {
    // TODO: add support for GitHub Enterprise Server hosts

    // Check if the user overrode the directory where GitHub CLI stores config files and load the config from that directory.
    // Otherwise, just load from `$XDG_CONFIG_HOME/gh`.
    if let Some(custom_gh_config_dir) = option_env!("GH_CONFIG_DIR") {
        let ghcli_config = GhCliConfig::load_from(custom_gh_config_dir)?;
        let ghcli_host_config = Hosts::load_from(custom_gh_config_dir)?
            .get(GITHUB_COM)
            .cloned()
            .unwrap();

        Ok((ghcli_config, ghcli_host_config))
    } else {
        let ghcli_config = GhCliConfig::load()?;
        let ghcli_host_config = Hosts::load()?.get(GITHUB_COM).cloned().unwrap();

        Ok((ghcli_config, ghcli_host_config))
    }
}

fn initialize_terminal() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    Ok(())
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;

    Ok(())
}

async fn ui_loop(instance: Arc<TokioMutex<Instance>>) -> Result<()> {
    initialize_terminal()?;

    let mut backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    terminal.show_cursor()?;
    restore_terminal()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(ghcom_auth_token) = retrieve_ghcom_pat_from_env() {
        let ghapi_client_builder = Octocrab::builder().personal_token(ghcom_auth_token.into());
        let ghapi_client = octocrab::initialise(ghapi_client_builder)?;

        let instance = Arc::new(TokioMutex::new(Instance::with_pat(ghapi_client)));

        ui_loop(instance).await?;
    }

    Ok(())
}
