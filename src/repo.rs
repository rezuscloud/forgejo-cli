use std::{io::Write, path::PathBuf};

use clap::Subcommand;
use eyre::{eyre, OptionExt};
use forgejo_api::structs::CreateRepoOption;
use url::Url;

use crate::SpecialRender;

pub struct RepoInfo {
    url: Url,
    name: Option<RepoName>,
}

impl RepoInfo {
    pub fn get_current(
        host: Option<&str>,
        repo: Option<&str>,
        remote: Option<&str>,
    ) -> eyre::Result<Self> {
        // l = domain/owner/name
        // s = owner/name
        // x = is present
        // i = found locally by git
        //
        // | repo | host | remote | ans-host | ans-repo |
        // |------|------|--------|----------|----------|
        // | l    | x    | x      | repo     | repo     |
        // | l    | x    | i      | repo     | repo     |
        // | l    | x    |        | repo     | repo     |
        // | l    |      | x      | repo     | repo     |
        // | l    |      | i      | repo     | repo     |
        // | l    |      |        | repo     | repo     |
        // | s    | x    | x      | host     | repo     |
        // | s    | x    | i      | host     | repo     |
        // | s    | x    |        | host     | repo     |
        // | s    |      | x      | remote   | repo     |
        // | s    |      | i      | remote   | repo     |
        // | s    |      |        | err      | repo     |
        // |      | x    | x      | remote   | remote   |
        // |      | x    | i      | host     | ?remote  |
        // |      | x    |        | host     | none     |
        // |      |      | x      | remote   | remote   |
        // |      |      | i      | remote   | remote   |
        // |      |      |        | err      | remote   |

        let mut repo_url: Option<Url> = None;
        let mut repo_name: Option<RepoName> = None;

        if let Some(repo) = repo {
            let (head, name) = repo
                .rsplit_once("/")
                .ok_or_eyre("repo name must contain owner and name")?;
            let name = name.strip_suffix(".git").unwrap_or(name);
            match head.rsplit_once("/") {
                Some((url, owner)) => {
                    if let Ok(url) = Url::parse(url) {
                        repo_url = Some(url);
                    } else if let Ok(url) = Url::parse(&format!("https://{url}/")) {
                        repo_url = Some(url);
                    }
                    repo_name = Some(RepoName {
                        owner: owner.to_owned(),
                        name: name.to_owned(),
                    });
                }
                None => {
                    repo_name = Some(RepoName {
                        owner: head.to_owned(),
                        name: name.to_owned(),
                    });
                }
            }
        }

        let repo_url = repo_url;
        let repo_name = repo_name;

        let host_url = host.and_then(|host| {
            Url::parse(host)
                .ok()
                .or_else(|| Url::parse(&format!("https://{host}/")).ok())
        });

        let (remote_url, remote_repo_name) = {
            let mut out = (None, None);
            if let Ok(local_repo) = git2::Repository::open(".") {
                // help to escape scopes
                let tmp;
                let mut tmp2;

                let mut name = remote;

                // if there's only one remote, use that
                if name.is_none() {
                    let all_remotes = local_repo.remotes()?;
                    if all_remotes.len() == 1 {
                        if let Some(remote_name) = all_remotes.get(0) {
                            tmp2 = Some(remote_name.to_owned());
                            name = tmp2.as_deref();
                        }
                    }
                }

                // if there's a remote whose host url matches the one
                // specified with `--host`, use that
                //
                // This is different than using `--host` itself, since this
                // will include the repo name, which `--host` can't do.
                if name.is_none() {
                    if let Some(host_url) = &host_url {
                        let all_remotes = local_repo.remotes()?;
                        for remote_name in all_remotes.iter() {
                            let Some(remote_name) = remote_name else {
                                continue;
                            };
                            let remote = local_repo.find_remote(remote_name)?;

                            if let Some(url) = remote.url() {
                                let (url, _) = url_strip_repo_name(Url::parse(url)?)?;
                                if url.host_str() == host_url.host_str()
                                    && url.path() == host_url.path()
                                {
                                    tmp2 = Some(remote_name.to_owned());
                                    name = tmp2.as_deref();
                                }
                            }
                        }
                    }
                }

                // if the current branch is tracking a remote branch, use that remote
                if name.is_none() {
                    let head = local_repo.head()?;
                    let branch_name = head.name().ok_or_else(|| eyre!("branch name not UTF-8"))?;
                    tmp = local_repo.branch_upstream_remote(branch_name).ok();
                    name = tmp
                        .as_ref()
                        .map(|remote| {
                            remote
                                .as_str()
                                .ok_or_else(|| eyre!("remote name not UTF-8"))
                        })
                        .transpose()?;
                }

                if let Some(name) = name {
                    if let Ok(remote) = local_repo.find_remote(name) {
                        let url_s = std::str::from_utf8(remote.url_bytes())?;
                        let url = Url::parse(url_s)?;
                        let (url, name) = url_strip_repo_name(url)?;

                        out = (Some(url), Some(name))
                    }
                }
            } else {
                eyre::ensure!(remote.is_none(), "remote specified but no git repo found");
            }
            out
        };

        let (url, name) = if repo_url.is_some() {
            (repo_url, repo_name)
        } else if repo_name.is_some() {
            (host_url.or(remote_url), repo_name)
        } else {
            if remote.is_some() {
                (remote_url, remote_repo_name)
            } else if host_url.is_none() || remote_url == host_url {
                (remote_url, remote_repo_name)
            } else {
                (host_url, None)
            }
        };

        let info = match (url, name) {
            (Some(url), name) => RepoInfo { url, name },
            (None, Some(_)) => eyre::bail!("cannot find repo, no host specified"),
            (None, None) => eyre::bail!("no repo info specified"),
        };

        Ok(info)
    }

    pub fn name(&self) -> Option<&RepoName> {
        self.name.as_ref()
    }

    pub fn host_url(&self) -> &Url {
        &self.url
    }
}

fn url_strip_repo_name(mut url: Url) -> eyre::Result<(Url, RepoName)> {
    let mut iter = url
        .path_segments()
        .ok_or_eyre("repo url cannot be a base")?
        .rev();

    let name = iter.next().ok_or_eyre("repo url too short")?;
    let name = name.strip_suffix(".git").unwrap_or(name).to_owned();

    let owner = iter.next().ok_or_eyre("repo url too short")?.to_owned();

    // Remove the username and repo name from the url
    url.path_segments_mut()
        .map_err(|_| eyre!("repo url cannot be a base"))?
        .pop()
        .pop();

    Ok((url, RepoName { owner, name }))
}

#[derive(Debug)]
pub struct RepoName {
    owner: String,
    name: String,
}

impl RepoName {
    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Subcommand, Clone, Debug)]
pub enum RepoCommand {
    Create {
        repo: String,

        // flags
        #[clap(long, short)]
        description: Option<String>,
        #[clap(long, short = 'P')]
        private: bool,
        /// Creates a new remote with the given name for the new repo
        #[clap(long, short)]
        remote: Option<String>,
        /// Pushes the current branch to the default branch on the new repo.
        /// Implies `--remote=origin` (setting remote manually overrides this)
        #[clap(long, short)]
        push: bool,
    },
    View {
        name: Option<String>,
        #[clap(long, short = 'R')]
        remote: Option<String>,
    },
    Clone {
        repo: String,
        path: Option<PathBuf>,
    },
    Star {
        repo: String,
    },
    Unstar {
        repo: String,
    },
    Browse {
        name: Option<String>,
        #[clap(long, short = 'R')]
        remote: Option<String>,
    },
}

impl RepoCommand {
    pub async fn run(self, keys: &mut crate::KeyInfo, host_name: Option<&str>) -> eyre::Result<()> {
        match self {
            RepoCommand::Create {
                repo,

                description,
                private,
                remote,
                push,
            } => {
                if remote.is_some() || push {
                    let repo = git2::Repository::open(".")?;

                    let upstream = remote.as_deref().unwrap_or("origin");
                    if repo.find_remote(upstream).is_ok() {
                        eyre::bail!("A remote named \"{upstream}\" already exists");
                    }
                }
                let host = RepoInfo::get_current(host_name, None, None)?;
                let api = keys.get_api(host.host_url()).await?;
                let repo_spec = CreateRepoOption {
                    auto_init: Some(false),
                    default_branch: Some("main".into()),
                    description,
                    gitignores: None,
                    issue_labels: None,
                    license: None,
                    name: repo.clone(),
                    object_format_name: None,
                    private: Some(private),
                    readme: Some(String::new()),
                    template: Some(false),
                    trust_model: Some(forgejo_api::structs::CreateRepoOptionTrustModel::Default),
                };
                let new_repo = api.create_current_user_repo(repo_spec).await?;
                let html_url = new_repo
                    .html_url
                    .as_ref()
                    .ok_or_else(|| eyre::eyre!("new_repo does not have html_url"))?;
                println!("created new repo at {}", html_url);

                if remote.is_some() || push {
                    let repo = git2::Repository::open(".")?;

                    let upstream = remote.as_deref().unwrap_or("origin");
                    let clone_url = new_repo
                        .clone_url
                        .as_ref()
                        .ok_or_else(|| eyre::eyre!("new_repo does not have clone_url"))?;
                    let mut remote = repo.remote(upstream, clone_url.as_str())?;

                    if push {
                        let head = repo.head()?;
                        if !head.is_branch() {
                            eyre::bail!("HEAD is not on a branch; cannot push to remote");
                        }
                        let branch_shorthand = head
                            .shorthand()
                            .ok_or_else(|| eyre!("branch name invalid utf-8"))?
                            .to_owned();
                        let branch_name = std::str::from_utf8(head.name_bytes())?.to_owned();

                        let auth = auth_git2::GitAuthenticator::new();
                        auth.push(&repo, &mut remote, &[&branch_name])?;

                        remote.fetch(&[&branch_shorthand], None, None)?;

                        let mut current_branch = git2::Branch::wrap(head);
                        current_branch
                            .set_upstream(Some(&format!("{upstream}/{branch_shorthand}")))?;
                    }
                }
            }
            RepoCommand::View { name, remote } => {
                let repo = RepoInfo::get_current(host_name, name.as_deref(), remote.as_deref())?;
                let api = keys.get_api(&repo.host_url()).await?;
                let repo = repo
                    .name()
                    .ok_or_eyre("couldn't get repo name, please specify")?;
                let repo = api.repo_get(repo.owner(), repo.name()).await?;

                let SpecialRender {
                    dash,
                    body_prefix,
                    dark_grey,
                    reset,
                    ..
                } = crate::special_render();

                println!("{}", repo.full_name.ok_or_eyre("no full name")?);

                if let Some(parent) = &repo.parent {
                    println!(
                        "Fork of {}",
                        parent.full_name.as_ref().ok_or_eyre("no full name")?
                    );
                }
                if repo.mirror == Some(true) {
                    if let Some(original) = &repo.original_url {
                        println!("Mirror of {original}")
                    }
                }
                let desc = repo.description.as_deref().unwrap_or_default();
                if !desc.is_empty() {
                    if desc.lines().count() > 1 {
                        println!();
                    }
                    for line in desc.lines() {
                        println!("{dark_grey}{body_prefix}{reset} {line}");
                    }
                }
                println!();

                let lang = repo.language.as_deref().unwrap_or_default();
                if !lang.is_empty() {
                    println!("Primary language is {lang}");
                }

                let stars = repo.stars_count.unwrap_or_default();
                if stars == 1 {
                    print!("{stars} star {dash} ");
                } else {
                    print!("{stars} stars {dash} ");
                }

                let watchers = repo.watchers_count.unwrap_or_default();
                print!("{watchers} watching {dash} ");

                let forks = repo.forks_count.unwrap_or_default();
                if forks == 1 {
                    print!("{forks} fork");
                } else {
                    print!("{forks} forks");
                }
                println!();

                let mut first = true;
                if repo.has_issues.unwrap_or_default() && repo.external_tracker.is_none() {
                    let issues = repo.open_issues_count.unwrap_or_default();
                    if issues == 1 {
                        print!("{issues} issue");
                    } else {
                        print!("{issues} issues");
                    }
                    first = false;
                }
                if repo.has_pull_requests.unwrap_or_default() {
                    if !first {
                        print!(" {dash} ");
                    }
                    let pulls = repo.open_pr_counter.unwrap_or_default();
                    if pulls == 1 {
                        print!("{pulls} PR");
                    } else {
                        print!("{pulls} PRs");
                    }
                    first = false;
                }
                if repo.has_releases.unwrap_or_default() {
                    if !first {
                        print!(" {dash} ");
                    }
                    let releases = repo.release_counter.unwrap_or_default();
                    if releases == 1 {
                        print!("{releases} release");
                    } else {
                        print!("{releases} releases");
                    }
                    first = false;
                }
                if !first {
                    println!();
                }
                if let Some(external_tracker) = &repo.external_tracker {
                    if let Some(tracker_url) = &external_tracker.external_tracker_url {
                        println!("Issue tracker is at {tracker_url}");
                    }
                }

                if let Some(html_url) = &repo.html_url {
                    println!();
                    println!("View online at {html_url}");
                }
            }
            RepoCommand::Clone { repo, path } => {
                let repo = RepoInfo::get_current(host_name, Some(&repo), None)?;
                let api = keys.get_api(&repo.host_url()).await?;
                let name = repo.name().unwrap();

                let repo_data = api.repo_get(name.owner(), name.name()).await?;
                let clone_url = repo_data
                    .clone_url
                    .as_ref()
                    .ok_or_eyre("repo does not have clone url")?;

                let repo_name = repo_data
                    .name
                    .as_deref()
                    .ok_or_eyre("repo does not have name")?;
                let repo_full_name = repo_data
                    .full_name
                    .as_deref()
                    .ok_or_eyre("repo does not have full name")?;

                let path = path.unwrap_or_else(|| PathBuf::from(format!("./{repo_name}")));

                let SpecialRender {
                    colors, // actually using it to indicate fanciness FIXME
                    hide_cursor,
                    show_cursor,
                    clear_line,
                    ..
                } = *crate::special_render();

                let auth = auth_git2::GitAuthenticator::new();
                let git_config = git2::Config::open_default()?;

                let mut options = git2::FetchOptions::new();
                let mut callbacks = git2::RemoteCallbacks::new();
                callbacks.credentials(auth.credentials(&git_config));

                if colors {
                    print!("{hide_cursor}");
                    print!("   Preparing...");
                    let _ = std::io::stdout().flush();

                    callbacks.transfer_progress(|progress| {
                        print!("{clear_line}\r");
                        if progress.received_objects() == progress.total_objects() {
                            if progress.indexed_deltas() == progress.total_deltas() {
                                print!("Finishing up...");
                            } else {
                                let percent = 100.0 * (progress.indexed_deltas() as f64)
                                    / (progress.total_deltas() as f64);
                                print!("   Resolving... {percent:.01}%");
                            }
                        } else {
                            let bytes = progress.received_bytes();
                            let percent = 100.0 * (progress.received_objects() as f64)
                                / (progress.total_objects() as f64);
                            print!(" Downloading... {percent:.01}%");
                            match bytes {
                                0..=1023 => print!(" ({}b)", bytes),
                                1024..=1048575 => print!(" ({:.01}kb)", (bytes as f64) / 1024.0),
                                1048576..=1073741823 => {
                                    print!(" ({:.01}mb)", (bytes as f64) / 1048576.0)
                                }
                                1073741824.. => {
                                    print!(" ({:.01}gb)", (bytes as f64) / 1073741824.0)
                                }
                            }
                        }
                        let _ = std::io::stdout().flush();
                        true
                    });
                    options.remote_callbacks(callbacks);
                }

                let local_repo = git2::build::RepoBuilder::new()
                    .fetch_options(options)
                    .clone(clone_url.as_str(), &path)?;
                if colors {
                    print!("{clear_line}{show_cursor}\r");
                }
                println!("Cloned {} into {}", repo_full_name, path.display());

                if let Some(parent) = repo_data.parent.as_deref() {
                    let parent_clone_url = parent
                        .clone_url
                        .as_ref()
                        .ok_or_eyre("parent repo does not have clone url")?;
                    local_repo.remote("upstream", parent_clone_url.as_str())?;
                }
            }
            RepoCommand::Star { repo } => {
                let repo = RepoInfo::get_current(host_name, Some(&repo), None)?;
                let api = keys.get_api(&repo.host_url()).await?;
                let name = repo.name().unwrap();
                api.user_current_put_star(name.owner(), name.name()).await?;
                println!("Starred {}/{}!", name.owner(), name.name());
            }
            RepoCommand::Unstar { repo } => {
                let repo = RepoInfo::get_current(host_name, Some(&repo), None)?;
                let api = keys.get_api(&repo.host_url()).await?;
                let name = repo.name().unwrap();
                api.user_current_delete_star(name.owner(), name.name())
                    .await?;
                println!("Removed star from {}/{}", name.owner(), name.name());
            }
            RepoCommand::Browse { name, remote } => {
                let repo = RepoInfo::get_current(host_name, name.as_deref(), remote.as_deref())?;
                let mut url = repo.host_url().clone();
                let repo = repo
                    .name()
                    .ok_or_eyre("couldn't get repo name, please specify")?;
                url.path_segments_mut()
                    .map_err(|_| eyre!("url invalid"))?
                    .extend([repo.owner(), repo.name()]);

                open::that(url.as_str())?;
            }
        };
        Ok(())
    }
}
