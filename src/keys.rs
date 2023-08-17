use std::{collections::BTreeMap, io::ErrorKind};
use eyre::eyre;
use tokio::io::AsyncWriteExt;
use url::Url;

#[derive(serde::Serialize, serde::Deserialize, Clone, Default)]
pub struct KeyInfo {
    pub hosts: BTreeMap<String, HostInfo>,
    pub domain_to_name: BTreeMap<String, String>,
}

impl KeyInfo {
    fn domain_to_name(&self, domain: &str) -> Option<&str> {
        self.domain_to_name.get(domain).map(|s| &**s)
    }

    pub async fn load() -> eyre::Result<Self> {
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

    pub async fn save(&self) -> eyre::Result<()> {
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

    pub async fn get_current_host_and_repo(&self) -> eyre::Result<(&str, &HostInfo, String)> {
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

    pub async fn get_current_host(&self) -> eyre::Result<(&str, &HostInfo)> {
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
pub struct HostInfo {
    pub default: Option<String>,
    pub url: Url,
    pub users: BTreeMap<String, UserInfo>,
}

impl HostInfo {
    pub fn get_current_user(&self) -> eyre::Result<(&str, &UserInfo)> {
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
pub struct UserInfo {
    pub name: String,
    pub key: String,
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
        eyre::bail!("could not find remote");
    };
    Ok(url)
}

