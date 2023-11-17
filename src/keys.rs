use eyre::eyre;
use std::{collections::BTreeMap, io::ErrorKind};
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

    pub fn get_api(&self, url: &Url) -> eyre::Result<forgejo_api::Forgejo> {
        self.get_login(url)?.api_for(url).map_err(Into::into)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Default)]
pub struct LoginInfo {
    name: String,
    key: String,
}

impl LoginInfo {
    pub fn new(name: String, key: String) -> Self {
        Self { name, key }
    }

    pub fn username(&self) -> &str {
        &self.name
    }

    pub fn api_for(&self, url: &Url) -> Result<forgejo_api::Forgejo, forgejo_api::ForgejoError> {
        forgejo_api::Forgejo::new(&self.key, url.clone())
    }
}
