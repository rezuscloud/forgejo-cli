use clap::{Args, Subcommand};

use crate::repo::RepoInfo;

#[derive(Args, Clone, Debug)]
pub struct OrgCommand {
    /// The local git remote that points to the repo to operate on.
    #[clap(long, short = 'R')]
    remote: Option<String>,
    #[clap(subcommand)]
    command: OrgSubcommand,
}

#[derive(Subcommand, Clone, Debug)]
pub enum OrgSubcommand {}

impl OrgCommand {
    pub async fn run(self, keys: &mut crate::KeyInfo, host_name: Option<&str>) -> eyre::Result<()> {
        let repo = RepoInfo::get_current(host_name, None, self.remote.as_deref(), &keys)?;
        let api = keys.get_api(repo.host_url()).await?;
        match self.command {}
    }
}
