use std::{collections::BTreeMap, io::ErrorKind};
use eyre::eyre;
use tokio::io::AsyncWriteExt;
use url::Url;

#[derive(serde::Serialize, serde::Deserialize, Clone, Default)]
pub struct KeyInfo {
    pub hosts: BTreeMap<String, LoginInfo>,
}

impl KeyInfo {
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

    pub fn get_current(&self) -> eyre::Result<(HostInfo<'_>, RepoInfo)> {
        let repo = git2::Repository::open(".")?;
        let remote_url = get_remote(&repo)?;
        let login_info = self.get_login(&remote_url)?;

        let mut path = remote_url.path_segments().ok_or_else(|| eyre!("bad path"))?.collect::<Vec<_>>();
        let repo_name = path.pop().ok_or_else(|| eyre!("path does not have repo name"))?.to_string();
        let owner = path.pop().ok_or_else(|| eyre!("path does not have owner name"))?.to_string();
        let base_path = path.join("/");

        let mut url = remote_url;
        url.set_path(&base_path);
        let host_info = HostInfo {
            url,
            login_info,
        };
        let repo_info = RepoInfo {
            owner,
            name: repo_name,
        };
        Ok((host_info, repo_info))
    }

    pub fn get_login(&self, url: &Url) -> eyre::Result<&LoginInfo> {
        let host_str = url
            .host_str()
            .ok_or_else(|| eyre!("remote url does not have host"))?;
        let domain = if let Some(port) = url.port() {
            format!("{}:{}", host_str, port)
        } else {
            host_str.to_owned()
        };

        let login_info = self
            .hosts
            .get(&domain)
            .ok_or_else(|| eyre!("not signed in to {domain}"))?;
        Ok(login_info)
    }
}

pub struct HostInfo<'a> {
    url: Url,
    login_info: &'a LoginInfo,
}

impl<'a> HostInfo<'a> {
    pub fn api(&self) -> Result<forgejo_api::Forgejo, forgejo_api::ForgejoError> {
        self.login_info.api_for(self.url())
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn username(&self) -> &'a str {
        &self.login_info.name
    }
}

pub struct RepoInfo {
    owner: String,
    name: String,
}

impl RepoInfo {
    pub fn owner(&self) -> &str {
        &self.owner
    }
    
    pub fn name(&self) -> &str {
        &self.name
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

#[derive(serde::Serialize, serde::Deserialize, Clone, Default)]
pub struct LoginInfo {
    name: String,
    key: String,
}

impl LoginInfo {
    pub fn new(name: String, key: String) -> Self {
        Self {
            name,
            key,
        }
    }

    pub fn username(&self) -> &str {
        &self.name
    }

    pub fn api_for(&self, url: &Url) -> Result<forgejo_api::Forgejo, forgejo_api::ForgejoError> {
        forgejo_api::Forgejo::new(&self.key, url.clone())
    }
}

fn get_remote(repo: &git2::Repository) -> eyre::Result<Url> {
    let head = repo.head()?;
    let branch_name = head.name().ok_or_else(|| eyre!("branch name not UTF-8"))?;
    let remote_name= repo.branch_upstream_remote(branch_name)?;
    let remote_name = remote_name.as_str().ok_or_else(|| eyre!("remote name not UTF-8"))?;
    let remote = repo.find_remote(remote_name)?;
    let url = Url::parse(std::str::from_utf8(remote.url_bytes())?)?;
    Ok(url)
}

