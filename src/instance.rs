use gh_config::{Config as GhCliConfig, Host as GhCliHostConfig};
use octocrab::Octocrab;
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
pub struct Instance {
    pub gh_client: Arc<Octocrab>,
    pub ghcli_config: Option<GhCliConfig>,
    pub ghcli_hosts_config: Option<GhCliHostConfig>,
    pub auth_prompt_on_start: bool,
}

impl Instance {
    /// The GitHub client instance will have already been supplied with the personal access token by the time
    /// when an instance of `Instance` is created.     
    pub fn with_pat(gh_client: Arc<Octocrab>) -> Self {
        Self {
            gh_client,
            ..Default::default()
        }
    }

    pub fn with_ghcli_config(
        gh_client: Arc<Octocrab>,
        ghcli_config: GhCliConfig,
        ghcli_hosts_config: GhCliHostConfig,
    ) -> Self {
        Self {
            gh_client,
            ghcli_config: Some(ghcli_config),
            ghcli_hosts_config: Some(ghcli_hosts_config),
            ..Default::default()
        }
    }

    pub fn with_auth_prompt() -> Self {
        Self {
            auth_prompt_on_start: true,
            ..Default::default()
        }
    }
}
