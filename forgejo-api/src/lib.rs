use serde::{de::DeserializeOwned, Serialize};
use url::Url;
use soft_assert::*;
use reqwest::{Client, StatusCode, Request};

pub struct Forgejo {
    url: Url,
    client: Client,
}

#[derive(thiserror::Error, Debug)]
pub enum ForgejoError {
    #[error("url must have a host")]
    HostRequired,
    #[error("scheme must be http or https")]
    HttpRequired,
    #[error("{0}")] // for some reason, you can't use `source` and `transparent` together
    ReqwestError(#[source] reqwest::Error),
    #[error("API key should be ascii")]
    KeyNotAscii,
    #[error("the response from forgejo was not properly structured")]
    BadStructure,
    #[error("unexpected status code {} {}", .0.as_u16(), .0.canonical_reason().unwrap_or(""))]
    UnexpectedStatusCode(StatusCode),
    #[error("{} {}: {}", .0.as_u16(), .0.canonical_reason().unwrap_or(""), .1)]
    ApiError(StatusCode, String)
}

impl From<reqwest::Error> for ForgejoError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_decode() {
            ForgejoError::BadStructure
        } else {
            ForgejoError::ReqwestError(e)
        }
    }
}

impl Forgejo {
    pub fn new(api_key: &str, url: Url) -> Result<Self, ForgejoError> { 
        Self::with_user_agent(api_key, url, "forgejo-api-rs")
    }

    pub fn with_user_agent(api_key: &str, url: Url, user_agent: &str) -> Result<Self, ForgejoError> {
        soft_assert!(matches!(url.scheme(), "http" | "https"), Err(ForgejoError::HttpRequired));

        let mut headers = reqwest::header::HeaderMap::new();
        let mut key_header: reqwest::header::HeaderValue = format!("token {api_key}").try_into().map_err(|_| ForgejoError::KeyNotAscii)?;
        // key_header.set_sensitive(true);
        headers.insert("Authorization", key_header);
        let client = Client::builder().user_agent(user_agent).default_headers(headers).build()?;
        dbg!(&client);
        Ok(Self { 
            url,
            client,
        })
    }

    pub async fn get_repo(&self, user: &str, repo: &str) -> Result<Option<Repo>, ForgejoError> {
        self.get_opt(&format!("repos/{user}/{repo}/")).await
    }

    pub async fn create_repo(&self, repo: CreateRepoOption) -> Result<Repo, ForgejoError> {
        self.post("user/repos", &repo).await
    }

    /// Returns user info about the authorized user.
    pub async fn myself(&self) -> Result<User, ForgejoError> {
        self.get("user").await
    }

    pub async fn get_user(&self, user: &str) -> Result<Option<User>, ForgejoError> {
        self.get_opt(&format!("users/{user}/")).await
    }

    pub async fn get_followers(&self, user: &str) -> Result<Option<Vec<User>>, ForgejoError> {
        self.get_opt(&format!("users/{user}/followers/")).await
    }

    pub async fn get_following(&self, user: &str) -> Result<Option<Vec<User>>, ForgejoError> {
        self.get_opt(&format!("users/{user}/following/")).await
    }

    async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, ForgejoError> {
        let url = self.url.join("api/v1/").unwrap().join(path).unwrap();
        let request = self.client.get(url).build()?;
        self.execute(request).await
    }

    async fn get_opt<T: DeserializeOwned>(&self, path: &str) -> Result<Option<T>, ForgejoError> {
        let url = self.url.join("api/v1/").unwrap().join(path).unwrap();
        let request = self.client.get(url).build()?;
        self.execute_opt(request).await
    }

    async fn post<T: Serialize, U: DeserializeOwned>(&self, path: &str, body: &T) -> Result<U, ForgejoError> {
        let url = self.url.join("api/v1/").unwrap().join(path).unwrap();
        let request = self.client.post(url).json(body).build()?;
        self.execute(request).await
    } 

    async fn execute<T: DeserializeOwned>(&self, request: Request) -> Result<T, ForgejoError> {
        let response = self.client.execute(dbg!(request)).await?;
        match response.status() {
            status if status.is_success() => Ok(response.json::<T>().await?),
            status if status.is_client_error() => Err(ForgejoError::ApiError(status, response.json::<ErrorMessage>().await?.message)),
            status => Err(ForgejoError::UnexpectedStatusCode(status))
        }
    }

    /// Like `execute`, but returns `Ok(None)` on 404.
    async fn execute_opt<T: DeserializeOwned>(&self, request: Request) -> Result<Option<T>, ForgejoError> {
        let response = self.client.execute(dbg!(request)).await?;
        match response.status() {
            status if status.is_success() => Ok(Some(response.json::<T>().await?)),
            StatusCode::NOT_FOUND => Ok(None),
            status if status.is_client_error() => Err(ForgejoError::ApiError(status, response.json::<ErrorMessage>().await?.message)),
            status => Err(ForgejoError::UnexpectedStatusCode(status))
        }
    }
}

#[derive(serde::Deserialize)]
struct ErrorMessage {
    message: String,
    // intentionally ignored, no need for now
    // url: Url 
}


#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Repo {
    pub clone_url: Url,
    #[serde(with="time::serde::rfc3339")]
    pub created_at: time::OffsetDateTime,
    pub default_branch: String,
    pub description: String,
    pub fork: bool,
    pub forks_count: u64,
    pub full_name: String,

    pub owner: User,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct User {
    pub active: bool,
    pub avatar_url: Url,
    #[serde(with="time::serde::rfc3339")]
    pub created: time::OffsetDateTime,
    pub description: String,
    pub email: String,
    pub followers_count: u64,
    pub following_count: u64,
    pub full_name: String,
    pub id: u64,
    pub is_admin: bool,
    pub language: String,
    #[serde(with="time::serde::rfc3339")]
    pub last_login: time::OffsetDateTime,
    pub location: String,
    pub login: String,
    pub login_name: String,
    pub prohibit_login: bool,
    pub restricted: bool,
    pub starred_repos_count: u64,
    pub website: String,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub enum UserVisibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "limited")]
    Limited,
    #[serde(rename = "private")]
    Private,
}

#[derive(serde::Serialize, Debug, PartialEq)]
pub struct CreateRepoOption {
    pub auto_init: bool,
    pub default_branch: String,
    pub description: Option<String>,
    pub gitignores: String,
    pub issue_labels: String,
    pub license: String,
    pub name: String,
    pub private: bool,
    pub readme: String,
    pub template: bool,
    pub trust_model: TrustModel
}

#[derive(serde::Serialize, Debug, PartialEq)]
pub enum TrustModel {
    Default,
    Collaborator,
    Committer,
    #[serde(rename = "collaboratorcommiter")]
    CollaboratorCommitter,
}