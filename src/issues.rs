use clap::Subcommand;
use eyre::eyre;
use forgejo_api::{Comment, CreateIssueCommentOption, EditIssueOption, Forgejo, IssueCommentQuery};

use crate::repo::RepoInfo;

#[derive(Subcommand, Clone, Debug)]
pub enum IssueCommand {
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
    View {
        id: u64,
        #[clap(subcommand)]
        command: Option<ViewCommand>,
    },
    Browse {
        id: Option<u64>,
    },
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
    pub async fn run(self, keys: &crate::KeyInfo) -> eyre::Result<()> {
        use IssueCommand::*;
        let repo = RepoInfo::get_current()?;
        let api = keys.get_api(&repo.host_url())?;
        match self {
            Create { title, body } => create_issue(&repo, &api, title, body).await?,
            View { id, command } => match command.unwrap_or(ViewCommand::Body) {
                ViewCommand::Body => view_issue(&repo, &api, id).await?,
                ViewCommand::Comment { idx } => view_comment(&repo, &api, id, idx).await?,
                ViewCommand::Comments => view_comments(&repo, &api, id).await?,
            },
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
    repo: &RepoInfo,
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
        .create_issue(
            repo.owner(),
            repo.name(),
            forgejo_api::CreateIssueOption {
                body: Some(body),
                title,
                ..Default::default()
            },
        )
        .await?;
    eprintln!("created issue #{}: {}", issue.number, issue.title);
    Ok(())
}

async fn view_issue(repo: &RepoInfo, api: &Forgejo, id: u64) -> eyre::Result<()> {
    let issue = api
        .get_issue(repo.owner(), repo.name(), id)
        .await?
        .ok_or_else(|| eyre!("issue {id} does not exist"))?;
    println!("#{}: {}", id, issue.title);
    println!("By {}", issue.user.login);
    if !issue.body.is_empty() {
        println!();
        println!("{}", issue.body);
    }
    Ok(())
}

async fn view_comment(repo: &RepoInfo, api: &Forgejo, id: u64, idx: usize) -> eyre::Result<()> {
    let comments = api
        .get_issue_comments(repo.owner(), repo.name(), id, IssueCommentQuery::default())
        .await?;
    let comment = comments
        .get(idx)
        .ok_or_else(|| eyre!("comment {idx} doesn't exist"))?;
    print_comment(&comment);
    Ok(())
}

async fn view_comments(repo: &RepoInfo, api: &Forgejo, id: u64) -> eyre::Result<()> {
    let comments = api
        .get_issue_comments(repo.owner(), repo.name(), id, IssueCommentQuery::default())
        .await?;
    for comment in comments {
        print_comment(&comment);
    }
    Ok(())
}

fn print_comment(comment: &Comment) {
    println!("{} said:", comment.user.login);
    println!("{}", comment.body);
    if !comment.assets.is_empty() {
        println!("({} attachments)", comment.assets.len());
    }
}

async fn browse_issue(repo: &RepoInfo, api: &Forgejo, id: Option<u64>) -> eyre::Result<()> {
    match id {
        Some(id) => {
            let issue = api
                .get_issue(repo.owner(), repo.name(), id)
                .await?
                .ok_or_else(|| eyre!("issue {id} does not exist"))?;
            open::that(issue.html_url.as_str())?;
        }
        None => {
            let repo = api
                .get_repo(repo.owner(), repo.name())
                .await?
                .ok_or_else(|| eyre!("repo {}/{} does not exist", repo.owner(), repo.name()))?;
            open::that(format!("{}/issues", repo.html_url))?;
        }
    }
    Ok(())
}

async fn add_comment(
    repo: &RepoInfo,
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
    api.create_comment(
        repo.owner(),
        repo.name(),
        issue,
        forgejo_api::CreateIssueCommentOption {
            body,
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

async fn edit_title(
    repo: &RepoInfo,
    api: &Forgejo,
    issue: u64,
    new_title: Option<String>,
) -> eyre::Result<()> {
    let new_title = match new_title {
        Some(s) => s,
        None => {
            let mut issue_info = api
                .get_issue(repo.owner(), repo.name(), issue)
                .await?
                .ok_or_else(|| eyre!("issue {issue} does not exist"))?;
            crate::editor(&mut issue_info.title, Some("md")).await?;
            issue_info.title
        }
    };
    if new_title.is_empty() {
        eyre::bail!("title cannot be empty");
    }
    if new_title.contains('\n') {
        eyre::bail!("title cannot contain newlines");
    }
    api.edit_issue(
        repo.owner(),
        repo.name(),
        issue,
        forgejo_api::EditIssueOption {
            title: Some(new_title.trim().to_owned()),
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

async fn edit_body(
    repo: &RepoInfo,
    api: &Forgejo,
    issue: u64,
    new_body: Option<String>,
) -> eyre::Result<()> {
    let new_body = match new_body {
        Some(s) => s,
        None => {
            let mut issue_info = api
                .get_issue(repo.owner(), repo.name(), issue)
                .await?
                .ok_or_else(|| eyre!("issue {issue} does not exist"))?;
            crate::editor(&mut issue_info.body, Some("md")).await?;
            issue_info.body
        }
    };
    api.edit_issue(
        repo.owner(),
        repo.name(),
        issue,
        forgejo_api::EditIssueOption {
            body: Some(new_body),
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

async fn edit_comment(
    repo: &RepoInfo,
    api: &Forgejo,
    issue: u64,
    idx: usize,
    new_body: Option<String>,
) -> eyre::Result<()> {
    let comments = api
        .get_issue_comments(
            repo.owner(),
            repo.name(),
            issue,
            forgejo_api::IssueCommentQuery::default(),
        )
        .await?;
    let comment = comments
        .get(idx)
        .ok_or_else(|| eyre!("comment not found"))?;
    let new_body = match new_body {
        Some(s) => s,
        None => {
            let mut body = comment.body.clone();
            crate::editor(&mut body, Some("md")).await?;
            body
        }
    };
    api.edit_comment(
        repo.owner(),
        repo.name(),
        comment.id,
        forgejo_api::EditIssueCommentOption {
            body: new_body,
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

async fn close_issue(
    repo: &RepoInfo,
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

        let opt = CreateIssueCommentOption { body };
        api.create_comment(repo.owner(), repo.name(), issue, opt)
            .await?;
    }

    let edit = EditIssueOption {
        state: Some(forgejo_api::State::Closed),
        ..Default::default()
    };
    api.edit_issue(repo.owner(), repo.name(), issue, edit)
        .await?;

    Ok(())
}
