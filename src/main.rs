use std::{collections::BTreeMap, io::ErrorKind};

use clap::{Parser, Subcommand};
use eyre::{bail, eyre};
use forgejo_api::{CreateRepoOption, Forgejo};
use tokio::io::AsyncWriteExt;
use url::Url;

mod keys;
use keys::*;

#[derive(Parser, Debug)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    #[clap(subcommand)]
    Repo(RepoCommand),
    User {
        #[clap(long, short)]
        host: Option<String>,
    },
    #[clap(subcommand)]
    Auth(AuthCommand),
}

#[derive(Subcommand, Clone, Debug)]
pub enum RepoCommand {
    Create {
        host: String,
        repo: String,

        // flags
        #[clap(long, short)]
        description: Option<String>,
        #[clap(long, short)]
        private: bool,
        /// Sets the new repo to be the `origin` remote of the current local repo.
        #[clap(long, short)]
        set_upstream: Option<String>,
        /// Pushes the current branch to the default branch on the new repo.
        /// Implies `--set-upstream=origin` (setting upstream manual overrides this)
        #[clap(long, short)]
        push: bool,
    },
    Info,
    Browse,
}

#[derive(Subcommand, Clone, Debug)]
pub enum AuthCommand {
    Login,
    Logout {
        host: String,
    },
    AddKey {
        /// The domain name of the forgejo instance.
        host: String,
        /// The user that the key is associated with
        user: String,
        /// The key to add. If not present, the key will be read in from stdin.
        key: Option<String>,
    },
    List,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args = App::parse();
    let mut keys = KeyInfo::load().await?;

    match args.command {
        Command::Repo(repo_subcommand) => match repo_subcommand {
            RepoCommand::Create {
                host,
                repo,

                description,
                private,
                set_upstream,
                push,
            } => {
                let host = Url::parse(&host)?;
                let login = keys.get_login(&host)?;
                let api = login.api_for(&host)?;
                let repo_spec = CreateRepoOption {
                    auto_init: false,
                    default_branch: "main".into(),
                    description,
                    gitignores: String::new(),
                    issue_labels: String::new(),
                    license: String::new(),
                    name: repo.clone(),
                    private,
                    readme: String::new(),
                    template: false,
                    trust_model: forgejo_api::TrustModel::Default,
                };
                let new_repo = api.create_repo(repo_spec).await?;
                eprintln!(
                    "created new repo at {}",
                    host.join(&format!("{}/{}", login.username(), repo))?
                );

                let upstream = set_upstream.as_deref().unwrap_or("origin");

                let repo = git2::Repository::open(".")?;
                let mut remote = if set_upstream.is_some() || push {
                    repo.remote(upstream, new_repo.clone_url.as_str())?
                } else {
                    repo.find_remote(upstream)?
                };

                if push {
                    remote.push::<&str>(&[], None)?;
                }
            }
            RepoCommand::Info => {
                let (host, repo) = keys.get_current()?;
                let api = host.api()?;
                let repo = api.get_repo(repo.owner(), repo.name()).await?;
                match repo {
                    Some(repo) => {
                        dbg!(repo);
                    }
                    None => eprintln!("repo not found"),
                }
            }
            RepoCommand::Browse => {
                let (host, repo) = keys.get_current()?;
                let mut url = host.url().clone();
                let new_path = format!("{}/{}/{}",
                    url.path()
                        .strip_suffix("/")
                        .unwrap_or(url.path()),
                    repo.owner(),
                    repo.name(),
                );
                url.set_path(&new_path);
                open::that(url.as_str())?;
            }
        },
        Command::User { host } => {
            let host = host.map(|host| Url::parse(&host)).transpose()?;
            let (url, name) = match host {
                Some(url) => (
                    keys.get_login(&url)?.username(),
                    url,
                ),
                None => {
                    let (host, _) = keys.get_current()?;
                    (host.username(), host.url().clone())
                }
            };
            eprintln!("currently signed in to {name}@{url}");
        }
        Command::Auth(auth_subcommand) => match auth_subcommand {
            AuthCommand::Login => {
                todo!();
                // let user = readline("username: ").await?;
                // let pass = readline("password: ").await?;
            }
            AuthCommand::Logout { host } => {
                let info_opt = keys
                    .hosts
                    .remove(&host);
                if let Some(info) = info_opt {
                    eprintln!("signed out of {}@{}", &info.username(), host);
                } else {
                    eprintln!("already not signed in to {host}");
                }
            }
            AuthCommand::AddKey {
                host,
                user,
                key,
            } => {
                let key = match key {
                    Some(key) => key,
                    None => readline("new key: ").await?,
                };
                if keys.hosts.get(&user).is_none() {
                    keys.hosts.insert(host, LoginInfo::new(user, key));
                } else {
                    println!(
                        "key for {} already exists",
                        host
                    );
                }
            }
            AuthCommand::List => {
                if keys.hosts.is_empty() {
                    println!("No logins.");
                }
                for (host_url, login_info) in &keys.hosts {
                    println!("{}@{}", login_info.username(), host_url);
                }
            }
        },
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

