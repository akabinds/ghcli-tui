use super::config::UserConfig;
use octocrab::Octocrab;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Instance {
    pub user_config: UserConfig,
    pub gh_client: Arc<Octocrab>,
}

impl Instance {
    pub fn new(user_config: UserConfig, gh_client: Arc<Octocrab>) -> Self {
        Self {
            user_config,
            gh_client,
        }
    }
}
