use clap::{Args, Subcommand};
use eyre::{eyre, OptionExt};
use forgejo_api::structs::{
    Comment, CreateIssueCommentOption, CreateIssueOption, EditIssueOption, IssueGetCommentsQuery,
};
use forgejo_api::Forgejo;

use crate::repo::{RepoInfo, RepoName};

#[derive(Args, Clone, Debug)]
pub struct IssueCommand {
    #[clap(long, short = 'R')]
    remote: Option<String>,
    #[clap(long, short)]
    repo: Option<String>,
    #[clap(subcommand)]
    command: IssueSubcommand,
}

#[derive(Subcommand, Clone, Debug)]
pub enum IssueSubcommand {
    Create {
        title: String,
        #[clap(long)]
        body: Option<String>,
    },
    Edit {
        issue: u64,
        #[clap(subcommand)]
        command: EditCommand,
    },
    Comment {
        issue: u64,
        body: Option<String>,
    },
    Close {
        issue: u64,
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
        state: Option<State>,
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
pub enum State {
    Open,
    Closed,
}

impl From<State> for forgejo_api::structs::IssueListIssuesQueryState {
    fn from(value: State) -> Self {
        match value {
            State::Open => forgejo_api::structs::IssueListIssuesQueryState::Open,
            State::Closed => forgejo_api::structs::IssueListIssuesQueryState::Closed,
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

impl IssueCommand {
    pub async fn run(self, keys: &crate::KeyInfo, host_name: Option<&str>) -> eyre::Result<()> {
        use IssueSubcommand::*;
        let repo = RepoInfo::get_current(host_name, self.repo.as_deref(), self.remote.as_deref())?;
        let api = keys.get_api(repo.host_url())?;
        let repo = repo
            .name()
            .ok_or_eyre("couldn't get repo name, try specifying with --repo")?;
        match self.command {
            Create { title, body } => create_issue(&repo, &api, title, body).await?,
            View { id, command } => match command.unwrap_or(ViewCommand::Body) {
                ViewCommand::Body => view_issue(&repo, &api, id).await?,
                ViewCommand::Comment { idx } => view_comment(&repo, &api, id, idx).await?,
                ViewCommand::Comments => view_comments(&repo, &api, id).await?,
            },
            Search {
                query,
                labels,
                creator,
                assignee,
                state,
            } => view_issues(&repo, &api, query, labels, creator, assignee, state).await?,
            Edit { issue, command } => match command {
                EditCommand::Title { new_title } => {
                    edit_title(&repo, &api, issue, new_title).await?
                }
                EditCommand::Body { new_body } => edit_body(&repo, &api, issue, new_body).await?,
                EditCommand::Comment { idx, new_body } => {
                    edit_comment(&repo, &api, issue, idx, new_body).await?
                }
            },
            Close { issue, with_msg } => close_issue(&repo, &api, issue, with_msg).await?,
            Browse { id } => browse_issue(&repo, &api, id).await?,
            Comment { issue, body } => add_comment(&repo, &api, issue, body).await?,
        }
        Ok(())
    }
}

async fn create_issue(
    repo: &RepoName,
    api: &Forgejo,
    title: String,
    body: Option<String>,
) -> eyre::Result<()> {
    let body = match body {
        Some(body) => body,
        None => {
            let mut body = String::new();
            crate::editor(&mut body, Some("md")).await?;
            body
        }
    };
    let issue = api
        .issue_create_issue(
            repo.owner(),
            repo.name(),
            CreateIssueOption {
                body: Some(body),
                title,
                assignee: None,
                assignees: None,
                closed: None,
                due_date: None,
                labels: None,
                milestone: None,
                r#ref: None,
            },
        )
        .await?;
    let number = issue
        .number
        .ok_or_else(|| eyre::eyre!("issue does not have number"))?;
    let title = issue
        .title
        .as_ref()
        .ok_or_else(|| eyre::eyre!("issue does not have title"))?;
    eprintln!("created issue #{}: {}", number, title);
    Ok(())
}

pub async fn view_issue(repo: &RepoName, api: &Forgejo, id: u64) -> eyre::Result<()> {
    let issue = api.issue_get_issue(repo.owner(), repo.name(), id).await?;
    let title = issue
        .title
        .as_ref()
        .ok_or_else(|| eyre::eyre!("issue does not have title"))?;
    let user = issue
        .user
        .as_ref()
        .ok_or_else(|| eyre::eyre!("issue does not have creator"))?;
    let username = user
        .login
        .as_ref()
        .ok_or_else(|| eyre::eyre!("user does not have login"))?;
    println!("#{}: {}", id, title);
    println!("By {}", username);
    if let Some(body) = &issue.body {
        println!();
        println!("{}", body);
    }
    Ok(())
}
async fn view_issues(
    repo: &RepoName,
    api: &Forgejo,
    query_str: Option<String>,
    labels: Option<String>,
    creator: Option<String>,
    assignee: Option<String>,
    state: Option<State>,
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
        r#type: None,
        milestones: None,
        since: None,
        before: None,
        mentioned_by: None,
        page: None,
        limit: None,
    };
    let issues = api
        .issue_list_issues(repo.owner(), repo.name(), query)
        .await?;
    if issues.len() == 1 {
        println!("1 issue");
    } else {
        println!("{} issues", issues.len());
    }
    for issue in issues {
        let number = issue
            .number
            .ok_or_else(|| eyre::eyre!("issue does not have number"))?;
        let title = issue
            .title
            .as_ref()
            .ok_or_else(|| eyre::eyre!("issue does not have title"))?;
        let user = issue
            .user
            .as_ref()
            .ok_or_else(|| eyre::eyre!("issue does not have creator"))?;
        let username = user
            .login
            .as_ref()
            .ok_or_else(|| eyre::eyre!("user does not have login"))?;
        println!("#{}: {} (by {})", number, title, username);
    }
    Ok(())
}

pub async fn view_comment(repo: &RepoName, api: &Forgejo, id: u64, idx: usize) -> eyre::Result<()> {
    let query = IssueGetCommentsQuery {
        since: None,
        before: None,
    };
    let comments = api
        .issue_get_comments(repo.owner(), repo.name(), id, query)
        .await?;
    let comment = comments
        .get(idx)
        .ok_or_else(|| eyre!("comment {idx} doesn't exist"))?;
    print_comment(&comment)?;
    Ok(())
}

pub async fn view_comments(repo: &RepoName, api: &Forgejo, id: u64) -> eyre::Result<()> {
    let query = IssueGetCommentsQuery {
        since: None,
        before: None,
    };
    let comments = api
        .issue_get_comments(repo.owner(), repo.name(), id, query)
        .await?;
    for comment in comments {
        print_comment(&comment)?;
    }
    Ok(())
}

fn print_comment(comment: &Comment) -> eyre::Result<()> {
    let body = comment
        .body
        .as_ref()
        .ok_or_else(|| eyre::eyre!("comment does not have body"))?;
    let user = comment
        .user
        .as_ref()
        .ok_or_else(|| eyre::eyre!("comment does not have user"))?;
    let username = user
        .login
        .as_ref()
        .ok_or_else(|| eyre::eyre!("user does not have login"))?;
    println!("{} said:", username);
    println!("{}", body);
    let assets = comment
        .assets
        .as_ref()
        .ok_or_else(|| eyre::eyre!("comment does not have assets"))?;
    if !assets.is_empty() {
        println!("({} attachments)", assets.len());
    }
    Ok(())
}

pub async fn browse_issue(repo: &RepoName, api: &Forgejo, id: Option<u64>) -> eyre::Result<()> {
    match id {
        Some(id) => {
            let issue = api.issue_get_issue(repo.owner(), repo.name(), id).await?;
            let html_url = issue
                .html_url
                .as_ref()
                .ok_or_else(|| eyre::eyre!("issue does not have html_url"))?;
            open::that(html_url.as_str())?;
        }
        None => {
            let repo = api.repo_get(repo.owner(), repo.name()).await?;
            let html_url = repo
                .html_url
                .as_ref()
                .ok_or_else(|| eyre::eyre!("issue does not have html_url"))?;
            open::that(format!("{}/issues", html_url))?;
        }
    }
    Ok(())
}

pub async fn add_comment(
    repo: &RepoName,
    api: &Forgejo,
    issue: u64,
    body: Option<String>,
) -> eyre::Result<()> {
    let body = match body {
        Some(body) => body,
        None => {
            let mut body = String::new();
            crate::editor(&mut body, Some("md")).await?;
            body
        }
    };
    api.issue_create_comment(
        repo.owner(),
        repo.name(),
        issue,
        forgejo_api::structs::CreateIssueCommentOption {
            body,
            updated_at: None,
        },
    )
    .await?;
    Ok(())
}

pub async fn edit_title(
    repo: &RepoName,
    api: &Forgejo,
    issue: u64,
    new_title: Option<String>,
) -> eyre::Result<()> {
    let new_title = match new_title {
        Some(s) => s,
        None => {
            let issue_info = api
                .issue_get_issue(repo.owner(), repo.name(), issue)
                .await?;
            let mut title = issue_info
                .title
                .ok_or_else(|| eyre::eyre!("issue does not have title"))?;
            crate::editor(&mut title, Some("md")).await?;
            title
        }
    };
    let new_title = new_title.trim();
    if new_title.is_empty() {
        eyre::bail!("title cannot be empty");
    }
    if new_title.contains('\n') {
        eyre::bail!("title cannot contain newlines");
    }
    api.issue_edit_issue(
        repo.owner(),
        repo.name(),
        issue,
        forgejo_api::structs::EditIssueOption {
            title: Some(new_title.to_owned()),
            assignee: None,
            assignees: None,
            body: None,
            due_date: None,
            milestone: None,
            r#ref: None,
            state: None,
            unset_due_date: None,
            updated_at: None,
        },
    )
    .await?;
    Ok(())
}

pub async fn edit_body(
    repo: &RepoName,
    api: &Forgejo,
    issue: u64,
    new_body: Option<String>,
) -> eyre::Result<()> {
    let new_body = match new_body {
        Some(s) => s,
        None => {
            let issue_info = api
                .issue_get_issue(repo.owner(), repo.name(), issue)
                .await?;
            let mut body = issue_info
                .body
                .ok_or_else(|| eyre::eyre!("issue does not have body"))?;
            crate::editor(&mut body, Some("md")).await?;
            body
        }
    };
    api.issue_edit_issue(
        repo.owner(),
        repo.name(),
        issue,
        forgejo_api::structs::EditIssueOption {
            body: Some(new_body),
            assignee: None,
            assignees: None,
            due_date: None,
            milestone: None,
            r#ref: None,
            state: None,
            title: None,
            unset_due_date: None,
            updated_at: None,
        },
    )
    .await?;
    Ok(())
}

pub async fn edit_comment(
    repo: &RepoName,
    api: &Forgejo,
    issue: u64,
    idx: usize,
    new_body: Option<String>,
) -> eyre::Result<()> {
    let comments = api
        .issue_get_comments(
            repo.owner(),
            repo.name(),
            issue,
            IssueGetCommentsQuery {
                since: None,
                before: None,
            },
        )
        .await?;
    let comment = comments
        .get(idx)
        .ok_or_else(|| eyre!("comment not found"))?;
    let new_body = match new_body {
        Some(s) => s,
        None => {
            let mut body = comment
                .body
                .clone()
                .ok_or_else(|| eyre::eyre!("issue does not have body"))?;
            crate::editor(&mut body, Some("md")).await?;
            body
        }
    };
    let id = comment
        .id
        .ok_or_else(|| eyre::eyre!("comment does not have id"))?;
    api.issue_edit_comment(
        repo.owner(),
        repo.name(),
        id,
        forgejo_api::structs::EditIssueCommentOption {
            body: new_body,
            updated_at: None,
        },
    )
    .await?;
    Ok(())
}

pub async fn close_issue(
    repo: &RepoName,
    api: &Forgejo,
    issue: u64,
    message: Option<Option<String>>,
) -> eyre::Result<()> {
    if let Some(message) = message {
        let body = match message {
            Some(m) => m,
            None => {
                let mut s = String::new();
                crate::editor(&mut s, Some("md")).await?;
                s
            }
        };

        let opt = CreateIssueCommentOption {
            body,
            updated_at: None,
        };
        api.issue_create_comment(repo.owner(), repo.name(), issue, opt)
            .await?;
    }

    let edit = EditIssueOption {
        state: Some("closed".into()),
        assignee: None,
        assignees: None,
        body: None,
        due_date: None,
        milestone: None,
        r#ref: None,
        title: None,
        unset_due_date: None,
        updated_at: None,
    };
    api.issue_edit_issue(repo.owner(), repo.name(), issue, edit)
        .await?;

    Ok(())
}
