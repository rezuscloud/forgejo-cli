use clap::{Args, Subcommand};

use crate::repo::{RepoArg, RepoInfo};

#[derive(Args, Clone, Debug)]
pub struct WikiCommand {
    /// The local git remote that points to the repo to operate on.
    #[clap(long, short = 'R')]
    remote: Option<String>,
    #[clap(subcommand)]
    command: WikiSubcommand,
}

#[derive(Subcommand, Clone, Debug)]
pub enum WikiSubcommand {}

impl WikiCommand {
    pub async fn run(self, keys: &mut crate::KeyInfo, host_name: Option<&str>) -> eyre::Result<()> {
        use WikiSubcommand::*;

        let repo = RepoInfo::get_current(host_name, self.repo(), self.remote.as_deref())?;
        let api = keys.get_api(repo.host_url()).await?;
        let repo = repo.name().ok_or_else(|| self.no_repo_error())?;

        match self.command {}
    }

    fn repo(&self) -> Option<&RepoArg> {
        use WikiSubcommand::*;
        match &self.command {
            _ => todo!(),
        }
    }

    fn no_repo_error(&self) -> eyre::Error {
        use WikiSubcommand::*;
        match &self.command {
            _ => todo!(),
        }
    }
}
