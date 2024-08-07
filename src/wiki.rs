use base64ct::Encoding;
use clap::{Args, Subcommand};
use eyre::{Context, OptionExt};
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
    Contents {
        repo: Option<RepoArg>,
    },
    View {
        #[clap(long, short)]
        repo: Option<RepoArg>,
        page: String,
    },
    Browse {
        #[clap(long, short)]
        repo: Option<RepoArg>,
        page: String,
    },
}

impl WikiCommand {
    pub async fn run(self, keys: &mut crate::KeyInfo, host_name: Option<&str>) -> eyre::Result<()> {
        use WikiSubcommand::*;

        let repo = RepoInfo::get_current(host_name, self.repo(), self.remote.as_deref())?;
        let api = keys.get_api(repo.host_url()).await?;
        let repo = repo.name().ok_or_else(|| self.no_repo_error())?;

        match self.command {
            Contents { repo: _ } => wiki_contents(&repo, &api).await?,
            View { repo: _, page } => view_wiki_page(&repo, &api, &*page).await?,
            Browse { repo: _, page } => browse_wiki_page(&repo, &api, &*page).await?,
        }
        Ok(())
    }

    fn repo(&self) -> Option<&RepoArg> {
        use WikiSubcommand::*;
        match &self.command {
            Contents { repo } | View { repo, .. } | Browse { repo, .. } => repo.as_ref(),
        }
    }

    fn no_repo_error(&self) -> eyre::Error {
        use WikiSubcommand::*;
        match &self.command {
            Contents { repo: _ } | View { .. } | Browse { .. } => {
                eyre::eyre!("couldn't guess repo")
            }
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

async fn view_wiki_page(repo: &RepoName, api: &Forgejo, page: &str) -> eyre::Result<()> {
    let SpecialRender { bold, reset, .. } = *crate::special_render();

    let page = api
        .repo_get_wiki_page(repo.owner(), repo.name(), page)
        .await?;

    let title = page
        .title
        .as_deref()
        .ok_or_eyre("page does not have title")?;
    println!("{bold}{title}{reset}");
    println!();

    let contents_b64 = page
        .content_base64
        .as_deref()
        .ok_or_eyre("page does not have content")?;
    let contents = String::from_utf8(base64ct::Base64::decode_vec(contents_b64)?)
        .wrap_err("page content is not utf-8")?;

    println!("{}", crate::markdown(&contents));
    Ok(())
}

async fn browse_wiki_page(repo: &RepoName, api: &Forgejo, page: &str) -> eyre::Result<()> {
    let page = api
        .repo_get_wiki_page(repo.owner(), repo.name(), page)
        .await?;
    let html_url = page
        .html_url
        .as_ref()
        .ok_or_eyre("page does not have html url")?;
    open::that(html_url.as_str())?;
    Ok(())
}
