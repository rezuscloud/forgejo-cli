use std::str::FromStr;

use clap::{Args, Subcommand};
use eyre::OptionExt;
use forgejo_api::{
    structs::{
        CreatePullRequestOption, MergePullRequestOption, RepoGetPullRequestCommitsQuery,
        RepoGetPullRequestFilesQuery,
    },
    Forgejo,
};

use crate::repo::{RepoInfo, RepoName};

#[derive(Args, Clone, Debug)]
pub struct PrCommand {
    #[clap(long, short = 'R')]
    remote: Option<String>,
    #[clap(long, short)]
    repo: Option<String>,
    #[clap(subcommand)]
    command: PrSubcommand,
}

#[derive(Subcommand, Clone, Debug)]
pub enum PrSubcommand {
    Create {
        base: String,
        head: String,
        title: String,
        #[clap(long)]
        body: Option<String>,
    },
    Edit {
        pr: u64,
        #[clap(subcommand)]
        command: EditCommand,
    },
    Merge {
        pr: u64,
        #[clap(long, short)]
        method: Option<MergeMethod>,
        #[clap(long, short)]
        delete: bool,
    },
    Comment {
        pr: u64,
        body: Option<String>,
    },
    Close {
        pr: u64,
        #[clap(long, short)]
        with_msg: Option<Option<String>>,
    },
    Search {
        query: Option<String>,
        #[clap(long, short)]
        labels: Option<String>,
        #[clap(long, short)]
        creator: Option<String>,
        #[clap(long, short)]
        assignee: Option<String>,
        #[clap(long, short)]
        state: Option<crate::issues::State>,
    },
    View {
        id: u64,
        #[clap(subcommand)]
        command: Option<ViewCommand>,
    },
    Checkout {
        /// The pull request to check out.
        ///
        /// Prefix with ^ to get a pull request from the parent repo.
        pr: PrNumber,
        /// The name to give the newly created branch.
        ///
        /// Defaults to naming after the host url, repo owner, and PR number.
        #[clap(long, id = "NAME")]
        branch_name: Option<String>,
    },
    Browse {
        id: Option<u64>,
    },
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum MergeMethod {
    Merge,
    Rebase,
    RebaseMerge,
    Squash,
    Manual,
}

#[derive(Clone, Copy, Debug)]
pub enum PrNumber {
    This(u64),
    Parent(u64),
}

impl PrNumber {
    fn number(self) -> u64 {
        match self {
            PrNumber::This(x) => x,
            PrNumber::Parent(x) => x,
        }
    }
}

impl FromStr for PrNumber {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(num) = s.strip_prefix("^") {
            Ok(Self::Parent(num.parse()?))
        } else {
            Ok(Self::This(s.parse()?))
        }
    }
}

impl From<MergeMethod> for forgejo_api::structs::MergePullRequestOptionDo {
    fn from(value: MergeMethod) -> Self {
        use forgejo_api::structs::MergePullRequestOptionDo::*;
        match value {
            MergeMethod::Merge => Merge,
            MergeMethod::Rebase => Rebase,
            MergeMethod::RebaseMerge => RebaseMerge,
            MergeMethod::Squash => Squash,
            MergeMethod::Manual => ManuallyMerged,
        }
    }
}

#[derive(Subcommand, Clone, Debug)]
pub enum EditCommand {
    Title {
        new_title: Option<String>,
    },
    Body {
        new_body: Option<String>,
    },
    Comment {
        idx: usize,
        new_body: Option<String>,
    },
}

#[derive(Subcommand, Clone, Debug)]
pub enum ViewCommand {
    Body,
    Comment {
        idx: usize,
    },
    Comments,
    Diff {
        /// Get the diff in patch format
        #[clap(long, short)]
        patch: bool,
        /// View the diff in your text editor
        #[clap(long, short)]
        editor: bool,
    },
    Files,
    Commits {
        /// View one commit per line
        #[clap(long, short)]
        oneline: bool,
    },
}

impl PrCommand {
    pub async fn run(self, keys: &crate::KeyInfo, host_name: Option<&str>) -> eyre::Result<()> {
        use PrSubcommand::*;
        let repo = RepoInfo::get_current(host_name, self.repo.as_deref(), self.remote.as_deref())?;
        let api = keys.get_api(repo.host_url())?;
        let repo = repo
            .name()
            .ok_or_eyre("couldn't get repo name, try specifying with --repo")?;
        match self.command {
            Create {
                title,
                base,
                head,
                body,
            } => create_pr(&repo, &api, title, base, head, body).await?,
            Merge { pr, method, delete } => merge_pr(&repo, &api, pr, method, delete).await?,
            View { id, command } => match command.unwrap_or(ViewCommand::Body) {
                ViewCommand::Body => view_pr(&repo, &api, id).await?,
                ViewCommand::Comment { idx } => {
                    crate::issues::view_comment(&repo, &api, id, idx).await?
                }
                ViewCommand::Comments => crate::issues::view_comments(&repo, &api, id).await?,
                ViewCommand::Diff { patch, editor } => {
                    view_diff(&repo, &api, id, patch, editor).await?
                }
                ViewCommand::Files => view_pr_files(&repo, &api, id).await?,
                ViewCommand::Commits { oneline } => {
                    view_pr_commits(&repo, &api, id, oneline).await?
                }
            },
            Search {
                query,
                labels,
                creator,
                assignee,
                state,
            } => view_prs(&repo, &api, query, labels, creator, assignee, state).await?,
            Edit { pr, command } => match command {
                EditCommand::Title { new_title } => {
                    crate::issues::edit_title(&repo, &api, pr, new_title).await?
                }
                EditCommand::Body { new_body } => {
                    crate::issues::edit_body(&repo, &api, pr, new_body).await?
                }
                EditCommand::Comment { idx, new_body } => {
                    crate::issues::edit_comment(&repo, &api, pr, idx, new_body).await?
                }
            },
            Close { pr, with_msg } => crate::issues::close_issue(&repo, &api, pr, with_msg).await?,
            Checkout { pr, branch_name } => {
                checkout_pr(&repo, &api, pr, self.repo.is_some(), branch_name).await?
            }
            Browse { id } => crate::issues::browse_issue(&repo, &api, id).await?,
            Comment { pr, body } => crate::issues::add_comment(&repo, &api, pr, body).await?,
        }
        Ok(())
    }
}

pub async fn view_pr(repo: &RepoName, api: &Forgejo, id: u64) -> eyre::Result<()> {
    let crate::SpecialRender {
        dash,
        body_prefix,

        bright_red,
        bright_green,
        bright_magenta,
        yellow,
        dark_grey,
        light_grey,
        white,
        reset,
        ..
    } = crate::special_render();

    let mut additions = 0;
    let mut deletions = 0;
    let query = RepoGetPullRequestFilesQuery {
        limit: Some(u32::MAX),
        ..Default::default()
    };
    let (_, files) = api
        .repo_get_pull_request_files(repo.owner(), repo.name(), id, query)
        .await?;
    for file in files {
        additions += file.additions.unwrap_or_default();
        deletions += file.deletions.unwrap_or_default();
    }
    let pr = api
        .repo_get_pull_request(repo.owner(), repo.name(), id)
        .await?;
    let title = pr
        .title
        .as_deref()
        .ok_or_else(|| eyre::eyre!("pr does not have title"))?;
    let title_no_wip = title
        .strip_prefix("WIP: ")
        .or_else(|| title.strip_prefix("WIP:"));
    let (title, is_draft) = match title_no_wip {
        Some(title) => (title, true),
        None => (title, false),
    };
    let state = pr
        .state
        .as_deref()
        .ok_or_else(|| eyre::eyre!("pr does not have state"))?;
    let state = match state {
        "open" if is_draft => format!("{light_grey}Draft{reset}"),
        "open" => format!("{bright_green}Open{reset}"),
        "closed" if pr.merged.unwrap_or_default() => format!("{bright_magenta}Merged{reset}"),
        "closed" => format!("{bright_red}Closed{reset}"),
        _ => "Unknown".to_owned(),
    };
    let base = pr.base.as_ref().ok_or_eyre("pr does not have base")?;
    let base_repo = base
        .repo
        .as_ref()
        .ok_or_eyre("base does not have repo")?
        .full_name
        .as_deref()
        .ok_or_eyre("base repo does not have name")?;
    let base_name = base
        .label
        .as_deref()
        .ok_or_eyre("base does not have label")?;
    let head = pr.head.as_ref().ok_or_eyre("pr does not have head")?;
    let head_repo = head
        .repo
        .as_ref()
        .ok_or_eyre("head does not have repo")?
        .full_name
        .as_deref()
        .ok_or_eyre("head repo does not have name")?;
    let head_name = head
        .label
        .as_deref()
        .ok_or_eyre("head does not have label")?;
    let head_name = if base_repo != head_repo {
        format!("{head_repo}:{head_name}")
    } else {
        head_name.to_owned()
    };
    let user = pr
        .user
        .as_ref()
        .ok_or_else(|| eyre::eyre!("pr does not have creator"))?;
    let username = user
        .login
        .as_ref()
        .ok_or_else(|| eyre::eyre!("user does not have login"))?;
    let comments = pr.comments.unwrap_or_default();
    println!("{yellow}{title}{reset} {dark_grey}#{id}{reset}");
    println!(
        "By {white}{username}{reset} {dash} {state} {dash} {bright_green}+{additions} {bright_red}-{deletions}{reset}"
    );
    println!("From `{head_name}` into `{base_name}`");

    if let Some(body) = &pr.body {
        if !body.trim().is_empty() {
            println!();
            for line in body.lines() {
                println!("{dark_grey}{body_prefix}{reset} {line}");
            }
        }
    }
    println!();
    if comments == 1 {
        println!("1 comment");
    } else {
        println!("{comments} comments");
    }
    Ok(())
}

async fn create_pr(
    repo: &RepoName,
    api: &Forgejo,
    title: String,
    base: String,
    head: String,
    body: Option<String>,
) -> eyre::Result<()> {
    let (repo_owner, repo_name, base, head) = match base.strip_prefix("^") {
        Some(parent_base) => {
            let mut repo_data = api.repo_get(repo.owner(), repo.name()).await?;
            let parent = *repo_data
                .parent
                .take()
                .ok_or_eyre("cannot create pull request upstream, there is no upstream")?;
            let parent_owner = parent
                .owner
                .ok_or_eyre("parent has no owner")?
                .login
                .ok_or_eyre("parent owner has no login")?;
            let parent_name = parent.name.ok_or_eyre("parent has no name")?;

            (
                parent_owner,
                parent_name,
                parent_base.to_owned(),
                format!("{}:{}", repo.owner(), head),
            )
        }
        None => (repo.owner().to_owned(), repo.name().to_owned(), base, head),
    };
    let body = match body {
        Some(body) => body,
        None => {
            let mut body = String::new();
            crate::editor(&mut body, Some("md")).await?;
            body
        }
    };
    let pr = api
        .repo_create_pull_request(
            &repo_owner,
            &repo_name,
            CreatePullRequestOption {
                assignee: None,
                assignees: None,
                base: Some(base.to_owned()),
                body: Some(body),
                due_date: None,
                head: Some(head),
                labels: None,
                milestone: None,
                title: Some(title),
            },
        )
        .await?;
    let number = pr
        .number
        .ok_or_else(|| eyre::eyre!("pr does not have number"))?;
    let title = pr
        .title
        .as_ref()
        .ok_or_else(|| eyre::eyre!("pr does not have title"))?;
    println!("created pull request #{}: {}", number, title);
    Ok(())
}

async fn merge_pr(
    repo: &RepoName,
    api: &Forgejo,
    pr: u64,
    method: Option<MergeMethod>,
    delete: bool,
) -> eyre::Result<()> {
    let repo_info = api.repo_get(repo.owner(), repo.name()).await?;
    let default_merge = repo_info
        .default_merge_style
        .map(|x| x.into())
        .unwrap_or(forgejo_api::structs::MergePullRequestOptionDo::Merge);
    let body = MergePullRequestOption {
        r#do: method.map(|x| x.into()).unwrap_or(default_merge),
        merge_commit_id: None,
        merge_message_field: None,
        merge_title_field: None,
        delete_branch_after_merge: Some(delete),
        force_merge: None,
        head_commit_id: None,
        merge_when_checks_succeed: None,
    };
    api.repo_merge_pull_request(repo.owner(), repo.name(), pr, body)
        .await?;
    Ok(())
}

async fn checkout_pr(
    repo: &RepoName,
    api: &Forgejo,
    pr: PrNumber,
    repo_specified: bool,
    branch_name: Option<String>,
) -> eyre::Result<()> {
    // this is so you don't checkout a pull request from an entirely different
    // repository. i.e. in this repo I could run
    // `fj pr -r codeberg.org/forgejo/forgejo checkout [num]` and have forgejo
    // appear in this repo.
    eyre::ensure!(
        !repo_specified,
        "Cannot checkout PR, `--repo` is not allowed when checking out a pull request"
    );

    let local_repo = git2::Repository::open(".").unwrap();

    let has_no_uncommited = local_repo.statuses(None).unwrap().is_empty();
    eyre::ensure!(
        has_no_uncommited,
        "Cannot checkout PR, working directory has uncommited changes"
    );

    let remote_repo = match pr {
        PrNumber::Parent(_) => {
            let mut this_repo = api.repo_get(repo.owner(), repo.name()).await?;
            let name = this_repo.full_name.as_deref().unwrap_or("???/???");
            *this_repo
                .parent
                .take()
                .ok_or_else(|| eyre::eyre!("cannot get parent repo, {name} is not a fork"))?
        }
        PrNumber::This(_) => api.repo_get(repo.owner(), repo.name()).await?,
    };

    let repo_owner = remote_repo
        .owner
        .as_ref()
        .ok_or_eyre("repo does not have owner")?
        .login
        .as_deref()
        .ok_or_eyre("owner does not have login")?;
    let repo_name = remote_repo
        .name
        .as_ref()
        .ok_or_eyre("repo does not have name")?;

    let pull_data = api
        .repo_get_pull_request(repo_owner, repo_name, pr.number())
        .await?;

    let url = remote_repo
        .clone_url
        .as_ref()
        .ok_or_eyre("repo has no clone url")?;
    let mut remote = local_repo.remote_anonymous(url.as_str())?;
    let branch_name = branch_name.unwrap_or_else(|| {
        format!(
            "pr-{}-{}-{}",
            url.host_str().unwrap_or("unknown"),
            repo_owner,
            pr.number(),
        )
    });

    auth_git2::GitAuthenticator::new().fetch(
        &local_repo,
        &mut remote,
        &[&format!("pull/{}/head", pr.number())],
        None,
    )?;

    let reference = local_repo.find_reference("FETCH_HEAD")?.resolve()?;
    let commit = reference.peel_to_commit()?;

    let mut branch_is_new = true;
    let branch =
        if let Ok(mut branch) = local_repo.find_branch(&branch_name, git2::BranchType::Local) {
            branch_is_new = false;
            branch
                .get_mut()
                .set_target(commit.id(), "update pr branch")?;
            branch
        } else {
            local_repo.branch(&branch_name, &commit, false)?
        };
    let branch_ref = branch
        .get()
        .name()
        .ok_or_eyre("branch does not have name")?;

    local_repo.set_head(branch_ref)?;
    local_repo
        // for some reason, `.force()` is required to make it actually update
        // file contents. thank you git2 examples for noticing this too, I would
        // have pulled out so much hair figuring this out myself.
        .checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
        .unwrap();

    let pr_title = pull_data.title.as_deref().ok_or_eyre("pr has no title")?;
    println!("Checked out PR #{}: {pr_title}", pr.number());
    if branch_is_new {
        println!("On new branch {branch_name}");
    } else {
        println!("Updated branch to latest commit");
    }

    Ok(())
}

async fn view_prs(
    repo: &RepoName,
    api: &Forgejo,
    query_str: Option<String>,
    labels: Option<String>,
    creator: Option<String>,
    assignee: Option<String>,
    state: Option<crate::issues::State>,
) -> eyre::Result<()> {
    let labels = labels
        .map(|s| s.split(',').map(|s| s.to_string()).collect::<Vec<_>>())
        .unwrap_or_default();
    let query = forgejo_api::structs::IssueListIssuesQuery {
        q: query_str,
        labels: Some(labels.join(",")),
        created_by: creator,
        assigned_by: assignee,
        state: state.map(|s| s.into()),
        r#type: Some(forgejo_api::structs::IssueListIssuesQueryType::Pulls),
        milestones: None,
        since: None,
        before: None,
        mentioned_by: None,
        page: None,
        limit: None,
    };
    let prs = api
        .issue_list_issues(repo.owner(), repo.name(), query)
        .await?;
    if prs.len() == 1 {
        println!("1 pull request");
    } else {
        println!("{} pull requests", prs.len());
    }
    for pr in prs {
        let number = pr
            .number
            .ok_or_else(|| eyre::eyre!("pr does not have number"))?;
        let title = pr
            .title
            .as_ref()
            .ok_or_else(|| eyre::eyre!("pr does not have title"))?;
        let user = pr
            .user
            .as_ref()
            .ok_or_else(|| eyre::eyre!("pr does not have creator"))?;
        let username = user
            .login
            .as_ref()
            .ok_or_else(|| eyre::eyre!("user does not have login"))?;
        println!("#{}: {} (by {})", number, title, username);
    }
    Ok(())
}

async fn view_diff(
    repo: &RepoName,
    api: &Forgejo,
    pr: u64,
    patch: bool,
    editor: bool,
) -> eyre::Result<()> {
    let diff_type = if patch { "patch" } else { "diff" };
    let diff = api
        .repo_download_pull_diff_or_patch(
            repo.owner(),
            repo.name(),
            pr,
            diff_type,
            forgejo_api::structs::RepoDownloadPullDiffOrPatchQuery::default(),
        )
        .await?;
    if editor {
        let mut view = diff.clone();
        crate::editor(&mut view, Some(diff_type)).await?;
        if view != diff {
            println!("changes made to the diff will not persist");
        }
    } else {
        println!("{diff}");
    }
    Ok(())
}

async fn view_pr_files(repo: &RepoName, api: &Forgejo, pr: u64) -> eyre::Result<()> {
    let crate::SpecialRender {
        bright_red,
        bright_green,
        reset,
        ..
    } = crate::special_render();

    let query = RepoGetPullRequestFilesQuery {
        limit: Some(u32::MAX),
        ..Default::default()
    };
    let (headers, files) = api
        .repo_get_pull_request_files(repo.owner(), repo.name(), pr, query)
        .await?;
    let max_additions = files
        .iter()
        .map(|x| x.additions.unwrap_or_default())
        .max()
        .unwrap_or_default();
    let max_deletions = files
        .iter()
        .map(|x| x.deletions.unwrap_or_default())
        .max()
        .unwrap_or_default();

    let additions_width = max_additions.checked_ilog10().unwrap_or_default() as usize + 1;
    let deletions_width = max_deletions.checked_ilog10().unwrap_or_default() as usize + 1;

    for file in files {
        let name = file.filename.as_deref().unwrap_or("???");
        let additions = file.additions.unwrap_or_default();
        let deletions = file.deletions.unwrap_or_default();
        println!("{bright_green}+{additions:<additions_width$} {bright_red}-{deletions:<deletions_width$}{reset} {name}");
    }
    Ok(())
}

async fn view_pr_commits(
    repo: &RepoName,
    api: &Forgejo,
    pr: u64,
    oneline: bool,
) -> eyre::Result<()> {
    let query = RepoGetPullRequestCommitsQuery {
        limit: Some(u32::MAX),
        files: Some(false),
        ..Default::default()
    };
    let (_headers, commits) = api
        .repo_get_pull_request_commits(repo.owner(), repo.name(), pr, query)
        .await?;

    let max_additions = commits
        .iter()
        .filter_map(|x| x.stats.as_ref())
        .map(|x| x.additions.unwrap_or_default())
        .max()
        .unwrap_or_default();
    let max_deletions = commits
        .iter()
        .filter_map(|x| x.stats.as_ref())
        .map(|x| x.deletions.unwrap_or_default())
        .max()
        .unwrap_or_default();

    let additions_width = max_additions.checked_ilog10().unwrap_or_default() as usize + 1;
    let deletions_width = max_deletions.checked_ilog10().unwrap_or_default() as usize + 1;

    let crate::SpecialRender {
        bright_red,
        bright_green,
        yellow,
        reset,
        ..
    } = crate::special_render();
    for commit in commits {
        let repo_commit = commit
            .commit
            .as_ref()
            .ok_or_eyre("commit does not have commit?")?;

        let message = repo_commit.message.as_deref().unwrap_or("[no msg]");
        let name = message.lines().next().unwrap_or(&message);

        let sha = commit
            .sha
            .as_deref()
            .ok_or_eyre("commit does not have sha")?;
        let short_sha = &sha[..7];

        let stats = commit
            .stats
            .as_ref()
            .ok_or_eyre("commit does not have stats")?;
        let additions = stats.additions.unwrap_or_default();
        let deletions = stats.deletions.unwrap_or_default();

        if oneline {
            println!("{yellow}{short_sha} {bright_green}+{additions:<additions_width$} {bright_red}-{deletions:<deletions_width$}{reset} {name}");
        } else {
            let author = repo_commit
                .author
                .as_ref()
                .ok_or_eyre("commit has no author")?;
            let author_name = author.name.as_deref().ok_or_eyre("author has no name")?;
            let author_email = author.email.as_deref().ok_or_eyre("author has no email")?;
            let date = commit
                .created
                .as_ref()
                .ok_or_eyre("commit as no creation date")?;

            println!("{yellow}commit {sha}{reset} ({bright_green}+{additions}{reset}, {bright_red}-{deletions}{reset})");
            println!("Author: {author_name} <{author_email}>");
            print!("Date:   ");
            let format = time::macros::format_description!("[weekday repr:short] [month repr:short] [day] [hour repr:24]:[minute]:[second] [year] [offset_hour sign:mandatory][offset_minute]");
            date.format_into(&mut std::io::stdout().lock(), format)?;
            println!();
            println!();
            for line in message.lines() {
                println!("    {line}");
            }
            println!();
        }
    }
    Ok(())
}
