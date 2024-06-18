use clap::{Args, Subcommand};

use crate::repo::RepoInfo;

#[derive(Args, Clone, Debug)]
pub struct UserCommand {
    #[clap(long, short = 'R')]
    remote: Option<String>,
    #[clap(subcommand)]
    command: UserSubcommand,
}

#[derive(Subcommand, Clone, Debug)]
pub enum UserSubcommand {}

impl UserCommand {
    pub async fn run(self, keys: &mut crate::KeyInfo, host_name: Option<&str>) -> eyre::Result<()> {
        let repo = RepoInfo::get_current(host_name, None, self.remote.as_deref())?;
        let api = keys.get_api(repo.host_url()).await?;
        match self.command {}
        Ok(())
    }
}
