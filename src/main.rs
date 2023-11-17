use std::{collections::BTreeMap, io::ErrorKind};

use clap::{Parser, Subcommand};
use eyre::{bail, eyre};
use forgejo_api::{CreateRepoOption, Forgejo};
use tokio::io::AsyncWriteExt;
use url::Url;

mod keys;
use keys::*;

mod auth;
mod repo;

#[derive(Parser, Debug)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    #[clap(subcommand)]
    Repo(repo::RepoCommand),
    User {
        #[clap(long, short)]
        host: Option<String>,
    },
    #[clap(subcommand)]
    Auth(auth::AuthCommand),
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args = App::parse();
    let mut keys = KeyInfo::load().await?;

    match args.command {
        Command::Repo(repo_subcommand) => repo_subcommand.run(&keys).await?,
        Command::User { host } => {
            let host = host.map(|host| Url::parse(&host)).transpose()?;
            let url = match host {
                Some(url) => url,
                None => {
                    repo::RepoInfo::get_current()?.url().clone()
                }
            };
            let name = keys.get_login(&url)?.username();
            eprintln!("currently signed in to {name}@{url}");
        }
        Command::Auth(auth_subcommand) => auth_subcommand.run(&mut keys).await?,
    }

    keys.save().await?;
    Ok(())
}

async fn readline(msg: &str) -> eyre::Result<String> {
    print!("{msg}");
    tokio::io::stdout().flush().await?;
    tokio::task::spawn_blocking(|| {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        Ok(input)
    })
    .await?
}

