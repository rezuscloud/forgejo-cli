use clap::Subcommand;
use eyre::eyre;
use forgejo_api::CreateRepoOption;
use url::Url;

pub struct RepoInfo {
    owner: String,
    name: String,
    url: Url,
}

impl RepoInfo {
    pub fn get_current() -> eyre::Result<Self> {
        let repo = git2::Repository::open(".")?;
        let url = get_remote(&repo)?;

        let mut path = url.path_segments().ok_or_else(|| eyre!("bad path"))?;
        let owner = path
            .next()
            .ok_or_else(|| eyre!("path does not have owner name"))?
            .to_string();
        let name = path
            .next()
            .ok_or_else(|| eyre!("path does not have repo name"))?;
        let name = name.strip_suffix(".git").unwrap_or(name).to_string();

        let repo_info = RepoInfo { owner, name, url };
        Ok(repo_info)
    }
    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn host_url(&self) -> Url {
        let mut url = self.url.clone();
        url.path_segments_mut()
            .expect("invalid url: cannot be a base")
            .pop()
            .pop();
        url
    }
}

fn get_remote(repo: &git2::Repository) -> eyre::Result<Url> {
    let head = repo.head()?;
    let branch_name = head.name().ok_or_else(|| eyre!("branch name not UTF-8"))?;
    let remote_name = repo.branch_upstream_remote(branch_name)?;
    let remote_name = remote_name
        .as_str()
        .ok_or_else(|| eyre!("remote name not UTF-8"))?;
    let remote = repo.find_remote(remote_name)?;
    let url = Url::parse(std::str::from_utf8(remote.url_bytes())?)?;
    Ok(url)
}

#[derive(Subcommand, Clone, Debug)]
pub enum RepoCommand {
    Create {
        host: String,
        repo: String,

        // flags
        #[clap(long, short)]
        description: Option<String>,
        #[clap(long, short = 'P')]
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

impl RepoCommand {
    pub async fn run(self, keys: &crate::KeyInfo) -> eyre::Result<()> {
        match self {
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

                if set_upstream.is_some() || push {
                    let repo = git2::Repository::open(".")?;

                    let upstream = set_upstream.as_deref().unwrap_or("origin");
                    let mut remote = repo.remote(upstream, new_repo.clone_url.as_str())?;

                    if push {
                        let head = repo.head()?;
                        if !head.is_branch() {
                            eyre::bail!("HEAD is not on a branch; cannot push to remote");
                        }
                        let branch_shorthand = head.shorthand().ok_or_else(|| eyre!("branch name invalid utf-8"))?.to_owned();
                        let branch_name = std::str::from_utf8(head.name_bytes())?.to_owned();
                        let mut current_branch = git2::Branch::wrap(head);
                        current_branch.set_upstream(Some(&dbg!(format!("{upstream}/{branch_shorthand}"))))?;

                        let auth = auth_git2::GitAuthenticator::new();
                        auth.push(&repo, &mut remote, &[&branch_name])?;
                    }
                }
            }
            RepoCommand::Info => {
                let repo = RepoInfo::get_current()?;
                let api = keys.get_api(&repo.host_url())?;
                let repo = api.get_repo(repo.owner(), repo.name()).await?;
                match repo {
                    Some(repo) => {
                        dbg!(repo);
                    }
                    None => eprintln!("repo not found"),
                }
            }
            RepoCommand::Browse => {
                let repo = RepoInfo::get_current()?;
                let mut url = repo.host_url().clone();
                let new_path = format!(
                    "{}/{}/{}",
                    url.path().strip_suffix("/").unwrap_or(url.path()),
                    repo.owner(),
                    repo.name(),
                );
                url.set_path(&new_path);
                open::that(url.as_str())?;
            }
        };
        Ok(())
    }
}
