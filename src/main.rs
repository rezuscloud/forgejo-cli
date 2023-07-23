use std::{collections::BTreeMap, io::ErrorKind};

use clap::{Parser, Subcommand};
use eyre::{bail, eyre};
use forgejo_api::{CreateRepoOption, Forgejo};
use tokio::io::AsyncWriteExt;
use url::Url;

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
        user: String,
    },
    Switch {
        /// The host to set the default account for.
        #[clap(short, long)]
        host: Option<String>,
        user: String,
    },
    AddKey {
        /// The domain name of the forgejo instance.
        host: String,
        /// The user that the key is associated with
        user: String,
        /// The name of the key. If not present, defaults to the username.
        #[clap(short, long)]
        name: Option<String>,
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
                // let (host_domain, host_keys, repo) = keys.get_current_host_and_repo().await?;
                let host_info = keys
                    .hosts
                    .get(&host)
                    .ok_or_else(|| eyre!("not a known host"))?;
                let (_, user) = host_info.get_current_user()?;
                let api = Forgejo::new(&user.key, host_info.url.clone())?;
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
                    host_info.url.join(&format!("{}/{}", user.name, repo))?
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
                let (_, host_keys, repo) = keys.get_current_host_and_repo().await?;
                let (_, user) = host_keys.get_current_user()?;
                let api = Forgejo::new(&user.key, host_keys.url.clone())?;
                let repo = api.get_repo(&user.name, &repo).await?;
                match repo {
                    Some(repo) => {
                        dbg!(repo);
                    }
                    None => eprintln!("repo not found"),
                }
            }
            RepoCommand::Browse => {
                let (_, host_keys, repo) = keys.get_current_host_and_repo().await?;
                let (_, user) = host_keys.get_current_user()?;
                open::that(
                    host_keys
                        .url
                        .join(&format!("/{}/{repo}", user.name))?
                        .as_str(),
                )?;
            }
        },
        Command::User { host } => {
            let (_, host_keys) = match host.as_deref() {
                Some(s) => (
                    s,
                    keys.hosts.get(s).ok_or_else(|| eyre!("not a known host"))?,
                ),
                None => keys.get_current_host().await?,
            };
            let (_, info) = host_keys.get_current_user()?;
            eprintln!("currently signed in to {}@{}", info.name, host_keys.url);
        }
        Command::Auth(auth_subcommand) => match auth_subcommand {
            AuthCommand::Login => {
                todo!();
                // let user = readline("username: ").await?;
                // let pass = readline("password: ").await?;
            }
            AuthCommand::Logout { host, user } => {
                let was_signed_in = keys
                    .hosts
                    .get_mut(&host)
                    .and_then(|host| host.users.remove(&user))
                    .is_some();
                if was_signed_in {
                    eprintln!("signed out of {user}@{host}");
                } else {
                    eprintln!("already not signed in");
                }
            }
            AuthCommand::Switch { host, user } => {
                let host = host.unwrap_or(keys.get_current_host().await?.0.to_string());
                let host_info = keys
                    .hosts
                    .get_mut(&host)
                    .ok_or_else(|| eyre!("not a known host"))?;
                if !host_info.users.contains_key(&user) {
                    bail!("could not switch user: not signed into {host} as {user}");
                }
                let previous = host_info.default.replace(user.clone());
                print!("set current user for {host} to {user}");
                match previous {
                    Some(prev) => println!(" (previously {prev})"),
                    None => println!(),
                }
            }
            AuthCommand::AddKey {
                host,
                user,
                name,
                key,
            } => {
                let host_keys = keys
                    .hosts
                    .get_mut(&host)
                    .ok_or_else(|| eyre!("unknown host {host}"))?;
                let key = match key {
                    Some(key) => key,
                    None => readline("new key: ").await?,
                };
                if host_keys.users.get(&user).is_none() {
                    host_keys.users.insert(
                        name.unwrap_or_else(|| user.clone()),
                        UserInfo { name: user, key },
                    );
                } else {
                    println!(
                        "key {} for {} already exists (rename it?)",
                        name.unwrap_or(user),
                        host
                    );
                }
            }
            AuthCommand::List => {
                if keys.hosts.is_empty() {
                    println!("No logins.");
                }
                for (host_url, host_info) in &keys.hosts {
                    for (key_name, key_info) in &host_info.users {
                        let UserInfo { name, key: _ } = key_info;
                        println!("{key_name}: {name}@{host_url}");
                    }
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

async fn get_remotes() -> eyre::Result<Vec<(String, Url)>> {
    let repo = git2::Repository::open(".")?;
    let remotes = repo
        .remotes()?
        .iter()
        .filter_map(|name| {
            let name = name?.to_string();
            let url = Url::parse(repo.find_remote(&name).ok()?.url()?).ok()?;
            Some((name, url))
        })
        .collect::<Vec<_>>();
    Ok(remotes)
}

async fn get_remote(remotes: &[(String, Url)]) -> eyre::Result<Url> {
    let url = if remotes.len() == 1 {
        remotes[0].1.clone()
    } else if let Some((_, url)) = remotes.iter().find(|(name, _)| *name == "origin") {
        url.clone()
    } else {
        bail!("could not find remote");
    };
    Ok(url)
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Default)]
struct KeyInfo {
    hosts: BTreeMap<String, HostInfo>,
    domain_to_name: BTreeMap<String, String>,
}

impl KeyInfo {
    fn domain_to_name(&self, domain: &str) -> Option<&str> {
        self.domain_to_name.get(domain).map(|s| &**s)
    }

    async fn load() -> eyre::Result<Self> {
        let path = directories::ProjectDirs::from("", "Cyborus", "forgejo-cli")
            .ok_or_else(|| eyre!("Could not find data directory"))?
            .data_dir()
            .join("keys.json");
        let json = tokio::fs::read(path).await;
        let this = match json {
            Ok(x) => serde_json::from_slice::<Self>(&x)?,
            Err(e) if e.kind() == ErrorKind::NotFound => {
                eprintln!("keys file not found, creating");
                Self::default()
            }
            Err(e) => return Err(e.into()),
        };
        Ok(this)
    }

    async fn save(&self) -> eyre::Result<()> {
        let json = serde_json::to_vec_pretty(self)?;
        let dirs = directories::ProjectDirs::from("", "Cyborus", "forgejo-cli")
            .ok_or_else(|| eyre!("Could not find data directory"))?;
        let path = dirs.data_dir();

        tokio::fs::create_dir_all(path).await?;

        tokio::fs::File::create(path.join("keys.json"))
            .await?
            .write_all(&json)
            .await?;

        Ok(())
    }

    async fn get_current_host_and_repo(&self) -> eyre::Result<(&str, &HostInfo, String)> {
        let remotes = get_remotes().await?;
        let remote = get_remote(&remotes).await?;
        let host_str = remote
            .host_str()
            .ok_or_else(|| eyre!("remote url does not have host"))?;
        let domain = if let Some(port) = remote.port() {
            format!("{}:{}", host_str, port)
        } else {
            host_str.to_owned()
        };
        let name = self
            .domain_to_name(&domain)
            .ok_or_else(|| eyre!("unknown remote"))?;

        let (name, host) = self
            .hosts
            .get_key_value(name)
            .ok_or_else(|| eyre!("not signed in to {domain}"))?;
        Ok((name, host, repo_from_url(&remote)?.into()))
    }

    async fn get_current_host(&self) -> eyre::Result<(&str, &HostInfo)> {
        let (name, host, _) = self.get_current_host_and_repo().await?;
        Ok((name, host))
    }

    async fn get_current_user(&self) -> eyre::Result<(&str, &UserInfo)> {
        let user = self.get_current_host().await?.1.get_current_user()?;

        Ok(user)
    }
}

fn repo_from_url(url: &Url) -> eyre::Result<&str> {
    let mut iter = url
        .path_segments()
        .ok_or_else(|| eyre!("failed to get path from url"))?;
    soft_assert::soft_assert!(
        matches!(iter.next(), Some(_)),
        Err(eyre!("path should have 2 segments, has none"))
    );
    let repo = iter
        .next()
        .ok_or_else(|| eyre!("path should have 2 segments, has only 1"))?;
    let repo = repo.strip_suffix(".git").unwrap_or(repo);
    soft_assert::soft_assert!(
        matches!(iter.next(), None),
        Err(eyre!("path should have 2 segments, has more"))
    );
    Ok(repo)
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct HostInfo {
    default: Option<String>,
    url: Url,
    users: BTreeMap<String, UserInfo>,
}

impl HostInfo {
    fn get_current_user(&self) -> eyre::Result<(&str, &UserInfo)> {
        if self.users.len() == 1 {
            let (s, k) = self.users.first_key_value().unwrap();
            return Ok((s, k));
        }
        if let Some(default) = self.default.as_ref() {
            if let Some(default_info) = self.users.get(default) {
                return Ok((default, default_info));
            }
        }

        Err(eyre!("could not find user"))
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Default)]
struct UserInfo {
    name: String,
    key: String,
}
