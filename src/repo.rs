use clap::Subcommand;
use url::Url;
use forgejo_api::CreateRepoOption;

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
