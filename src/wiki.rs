use clap::{Args, Subcommand};
use eyre::OptionExt;
use forgejo_api::Forgejo;

use crate::{
    repo::{RepoArg, RepoInfo, RepoName},
    SpecialRender,
};

#[derive(Args, Clone, Debug)]
pub struct WikiCommand {
    /// The local git remote that points to the repo to operate on.
    #[clap(long, short = 'R')]
    remote: Option<String>,
    #[clap(subcommand)]
    command: WikiSubcommand,
}

#[derive(Subcommand, Clone, Debug)]
pub enum WikiSubcommand {
    Contents { repo: Option<RepoArg> },
}

impl WikiCommand {
    pub async fn run(self, keys: &mut crate::KeyInfo, host_name: Option<&str>) -> eyre::Result<()> {
        use WikiSubcommand::*;

        let repo = RepoInfo::get_current(host_name, self.repo(), self.remote.as_deref())?;
        let api = keys.get_api(repo.host_url()).await?;
        let repo = repo.name().ok_or_else(|| self.no_repo_error())?;

        match self.command {
            Contents { repo: _ } => wiki_contents(&repo, &api).await?,
        }
        Ok(())
    }

    fn repo(&self) -> Option<&RepoArg> {
        use WikiSubcommand::*;
        match &self.command {
            Contents { repo } => repo.as_ref(),
        }
    }

    fn no_repo_error(&self) -> eyre::Error {
        use WikiSubcommand::*;
        match &self.command {
            Contents { repo: _ } => eyre::eyre!("couldn't guess repo"),
        }
    }
}

async fn wiki_contents(repo: &RepoName, api: &Forgejo) -> eyre::Result<()> {
    let SpecialRender { bullet, .. } = *crate::special_render();

    let query = forgejo_api::structs::RepoGetWikiPagesQuery {
        page: None,
        limit: None,
    };
    let pages = api
        .repo_get_wiki_pages(repo.owner(), repo.name(), query)
        .await?;
    for page in pages {
        let title = page
            .title
            .as_deref()
            .ok_or_eyre("page does not have title")?;
        println!("{bullet} {title}");
    }

    Ok(())
}
