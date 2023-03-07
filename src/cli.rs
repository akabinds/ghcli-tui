use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Ensure that an authentication prompt is shown when the UI starts. By default, this app
    /// assumes that you are wanting to use the GitHub CLI for personal use. But, if you are wanting
    /// to use the GitHub CLI through an organization or other user, it is recommended to authenticate
    /// as a GitHub app. Enabling this flag will allow you to do just that. In addition, if it is
    /// determined that you have never used GitHub CLI before using this TUI, which can be determined
    /// by if you have set the `GH_TOKEN` or `GITHUB_TOKEN` environment variables, or if GitHub CLI configuration
    /// isn't present in `$XDG_CONFIG_HOME/gh`, an autentication prompt will appear without having to
    /// use this flag.   
    #[arg(long = "auth-prompt")]
    pub auth_prompt_on_start: bool,

    /// Pass a path to a file that contains your GitHub personal access token. If the `GH_TOKEN` or `GITHUB_TOKEN`
    /// environment variables aren't set, you might be storing your personall access token in a file. This option
    /// allows you to specify the path to that file so that it can be passed to the `--with-token` option of
    /// `gh auth login` if needed.
    ///
    /// Read more about GitHub personal access tokens [here](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token).
    #[arg(long = "token-file")]
    pub token_file_path: Option<PathBuf>,
}
