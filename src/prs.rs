use clap::{Args, Subcommand};
use eyre::OptionExt;
use forgejo_api::{
    structs::{CreatePullRequestOption, MergePullRequestOption},
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
    Comment { idx: usize },
    Comments,
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
                ViewCommand::Body => crate::issues::view_issue(&repo, &api, id).await?,
                ViewCommand::Comment { idx } => {
                    crate::issues::view_comment(&repo, &api, id, idx).await?
                }
                ViewCommand::Comments => crate::issues::view_comments(&repo, &api, id).await?,
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
            Browse { id } => crate::issues::browse_issue(&repo, &api, id).await?,
            Comment { pr, body } => crate::issues::add_comment(&repo, &api, pr, body).await?,
        }
        Ok(())
    }
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
