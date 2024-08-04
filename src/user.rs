use clap::{Args, Subcommand};
use eyre::OptionExt;
use forgejo_api::Forgejo;

use crate::{repo::RepoInfo, SpecialRender};

#[derive(Args, Clone, Debug)]
pub struct UserCommand {
    #[clap(long, short = 'R')]
    remote: Option<String>,
    #[clap(subcommand)]
    command: UserSubcommand,
}

#[derive(Subcommand, Clone, Debug)]
pub enum UserSubcommand {
    /// Search for a user by username
    Search {
        /// The name to search for
        query: String,
        #[clap(long, short)]
        page: Option<usize>,
    },
    /// View a user's profile page
    View {
        /// The name of the user to view
        ///
        /// Omit to view your own page
        user: Option<String>,
    },
    /// Open a user's profile page in your browser
    Browse {
        /// The name of the user to open in your browser
        ///
        /// Omit to view your own page
        user: Option<String>,
    },
    /// Follow a user
    Follow {
        /// The name of the user to follow
        user: String,
    },
    /// Unfollow a user
    Unfollow {
        /// The name of the user to follow
        user: String,
    },
    /// List everyone a user's follows
    Following {
        /// The name of the user whose follows to list
        ///
        /// Omit to view your own follows
        user: Option<String>,
    },
    /// List a user's followers
    Followers {
        /// The name of the user whose followers to list
        ///
        /// Omit to view your own followers
        user: Option<String>,
    },
    /// Block a user
    Block {
        /// The name of the user to block
        user: String,
    },
    /// Unblock a user
    Unblock {
        /// The name of the user to unblock
        user: String,
    },
    /// List a user's repositories
    Repos {
        /// The name of the user whose repos to list
        ///
        /// Omit to view your own repos.
        user: Option<String>,
        /// List starred repos instead of owned repos
        #[clap(long)]
        starred: bool,
        /// Method by which to sort the list
        #[clap(long)]
        sort: Option<RepoSortOrder>,
    },
    /// List the organizations a user is a member of
    Orgs {
        /// The name of the user to view org membership of
        ///
        /// Omit to view your own orgs.
        user: Option<String>,
    },
    /// List a user's recent activity
    Activity {
        /// The name of the user to view the activity of
        ///
        /// Omit to view your own activity.
        user: Option<String>,
    },
    /// Edit your user settings
    #[clap(subcommand)]
    Edit(EditCommand),
}

#[derive(Subcommand, Clone, Debug)]
pub enum EditCommand {
    /// Set your bio
    Bio {
        /// The new description. Leave this out to open your editor.
        content: Option<String>,
    },
    /// Set your full name
    Name {
        /// The new name.
        #[clap(group = "arg")]
        name: Option<String>,
        /// Remove your name from your profile
        #[clap(long, short, group = "arg")]
        unset: bool,
    },
    /// Set your pronouns
    Pronouns {
        /// The new pronouns.
        #[clap(group = "arg")]
        pronouns: Option<String>,
        /// Remove your pronouns from your profile
        #[clap(long, short, group = "arg")]
        unset: bool,
    },
    /// Set your activity visibility
    Location {
        /// The new location.
        #[clap(group = "arg")]
        location: Option<String>,
        /// Remove your location from your profile
        #[clap(long, short, group = "arg")]
        unset: bool,
    },
    /// Set your activity visibility
    Activity {
        /// The visibility of your activity.
        #[clap(long, short)]
        visibility: VisbilitySetting,
    },
    /// Manage the email addresses associated with your account
    Email {
        /// Set the visibility of your email address.
        #[clap(long, short)]
        visibility: Option<VisbilitySetting>,
        /// Add a new email address
        #[clap(long, short)]
        add: Vec<String>,
        /// Remove an email address
        #[clap(long, short)]
        rm: Vec<String>,
    },
    /// Set your linked website
    Website {
        /// Your website URL.
        #[clap(group = "arg")]
        url: Option<String>,
        /// Remove your website from your profile
        #[clap(long, short, group = "arg")]
        unset: bool,
    },
}

#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum VisbilitySetting {
    Hidden,
    Public,
}

impl UserCommand {
    pub async fn run(self, keys: &mut crate::KeyInfo, host_name: Option<&str>) -> eyre::Result<()> {
        let repo = RepoInfo::get_current(host_name, None, self.remote.as_deref())?;
        let api = keys.get_api(repo.host_url()).await?;
        match self.command {
            UserSubcommand::Search { query, page } => user_search(&api, &query, page).await?,
            UserSubcommand::View { user } => view_user(&api, user.as_deref()).await?,
            UserSubcommand::Browse { user } => {
                browse_user(&api, repo.host_url(), user.as_deref()).await?
            }
            UserSubcommand::Follow { user } => follow_user(&api, &user).await?,
            UserSubcommand::Unfollow { user } => unfollow_user(&api, &user).await?,
            UserSubcommand::Following { user } => list_following(&api, user.as_deref()).await?,
            UserSubcommand::Followers { user } => list_followers(&api, user.as_deref()).await?,
            UserSubcommand::Block { user } => block_user(&api, &user).await?,
            UserSubcommand::Unblock { user } => unblock_user(&api, &user).await?,
            UserSubcommand::Repos {
                user,
                starred,
                sort,
            } => list_repos(&api, user.as_deref(), starred, sort).await?,
            UserSubcommand::Orgs { user } => list_orgs(&api, user.as_deref()).await?,
            UserSubcommand::Activity { user } => list_activity(&api, user.as_deref()).await?,
            UserSubcommand::Edit(cmd) => match cmd {
                EditCommand::Bio { content } => edit_bio(&api, content).await?,
                EditCommand::Name { name, unset } => edit_name(&api, name, unset).await?,
                EditCommand::Pronouns { pronouns, unset } => {
                    edit_pronouns(&api, pronouns, unset).await?
                }
                EditCommand::Location { location, unset } => {
                    edit_location(&api, location, unset).await?
                }
                EditCommand::Activity { visibility } => edit_activity(&api, visibility).await?,
                EditCommand::Email {
                    visibility,
                    add,
                    rm,
                } => edit_email(&api, visibility, add, rm).await?,
                EditCommand::Website { url, unset } => edit_website(&api, url, unset).await?,
            },
        }
        Ok(())
    }
}

async fn user_search(api: &Forgejo, query: &str, page: Option<usize>) -> eyre::Result<()> {
    let page = page.unwrap_or(1);
    if page == 0 {
        println!("There is no page 0");
    }
    let query = forgejo_api::structs::UserSearchQuery {
        q: Some(query.to_owned()),
        ..Default::default()
    };
    let result = api.user_search(query).await?;
    let users = result.data.ok_or_eyre("search did not return data")?;
    let ok = result.ok.ok_or_eyre("search did not return ok")?;
    if !ok {
        println!("Search failed");
        return Ok(());
    }
    if users.is_empty() {
        println!("No users matched that query");
    } else {
        let SpecialRender {
            bullet,
            dash,
            bold,
            reset,
            ..
        } = *crate::special_render();
        let page_start = (page - 1) * 20;
        let pages_total = users.len().div_ceil(20);
        if page_start >= users.len() {
            if pages_total == 1 {
                println!("There is only 1 page");
            } else {
                println!("There are only {pages_total} pages");
            }
        } else {
            for user in users.iter().skip(page_start).take(20) {
                let username = user
                    .login
                    .as_deref()
                    .ok_or_eyre("user does not have name")?;
                println!("{bullet} {bold}{username}{reset}");
            }
            println!(
                "Showing {bold}{}{dash}{}{reset} of {bold}{}{reset} results ({page}/{pages_total})",
                page_start + 1,
                (page_start + 20).min(users.len()),
                users.len()
            );
            if users.len() > 20 {
                println!("View more with the --page flag");
            }
        }
    }
    Ok(())
}

async fn view_user(api: &Forgejo, user: Option<&str>) -> eyre::Result<()> {
    let SpecialRender {
        bold,
        dash,
        bright_cyan,
        light_grey,
        reset,
        ..
    } = *crate::special_render();

    let user_data = match user {
        Some(user) => api.user_get(user).await?,
        None => api.user_get_current().await?,
    };
    let username = user_data
        .login
        .as_deref()
        .ok_or_eyre("user has no username")?;
    print!("{bright_cyan}{bold}{username}{reset}");
    if let Some(pronouns) = user_data.pronouns.as_deref() {
        if !pronouns.is_empty() {
            print!("{light_grey} {dash} {bold}{pronouns}{reset}");
        }
    }
    println!();
    let followers = user_data.followers_count.unwrap_or_default();
    let following = user_data.following_count.unwrap_or_default();
    println!("{bold}{followers}{reset} followers {dash} {bold}{following}{reset} following");
    let mut first = true;
    if let Some(website) = user_data.website.as_deref() {
        if !website.is_empty() {
            print!("{bold}{website}{reset}");
            first = false;
        }
    }
    if let Some(email) = user_data.email.as_deref() {
        if !email.is_empty() && !email.contains("noreply") {
            if !first {
                print!(" {dash} ");
            }
            print!("{bold}{email}{reset}");
        }
    }
    if !first {
        println!();
    }

    if let Some(desc) = user_data.description.as_deref() {
        if !desc.is_empty() {
            println!();
            println!("{}", crate::markdown(desc));
            println!();
        }
    }

    let joined = user_data
        .created
        .ok_or_eyre("user does not have join date")?;
    let date_format = time::macros::format_description!("[month repr:short] [day], [year]");
    println!("Joined on {bold}{}{reset}", joined.format(&date_format)?);

    Ok(())
}

async fn browse_user(api: &Forgejo, host_url: &url::Url, user: Option<&str>) -> eyre::Result<()> {
    let username = match user {
        Some(user) => user.to_owned(),
        None => {
            let myself = api.user_get_current().await?;
            myself
                .login
                .ok_or_eyre("authenticated user does not have login")?
        }
    };
    // `User` doesn't have an `html_url` field, so we gotta construct the user
    // page url ourselves
    let mut url = host_url.clone();
    url.path_segments_mut()
        .map_err(|_| eyre::eyre!("invalid host url"))?
        .push(&username);
    open::that(url.as_str())?;

    Ok(())
}

async fn follow_user(api: &Forgejo, user: &str) -> eyre::Result<()> {
    api.user_current_put_follow(user).await?;
    println!("Followed {user}");
    Ok(())
}

async fn unfollow_user(api: &Forgejo, user: &str) -> eyre::Result<()> {
    api.user_current_delete_follow(user).await?;
    println!("Unfollowed {user}");
    Ok(())
}

async fn list_following(api: &Forgejo, user: Option<&str>) -> eyre::Result<()> {
    let following = match user {
        Some(user) => {
            let query = forgejo_api::structs::UserListFollowingQuery {
                limit: Some(u32::MAX),
                ..Default::default()
            };
            api.user_list_following(user, query).await?
        }
        None => {
            let query = forgejo_api::structs::UserCurrentListFollowingQuery {
                limit: Some(u32::MAX),
                ..Default::default()
            };
            api.user_current_list_following(query).await?
        }
    };

    if following.is_empty() {
        match user {
            Some(name) => println!("{name} isn't following anyone"),
            None => println!("You aren't following anyone"),
        }
    } else {
        match user {
            Some(name) => println!("{name} is following:"),
            None => println!("You are following:"),
        }
        let SpecialRender { bullet, .. } = *crate::special_render();

        for followed in following {
            let username = followed
                .login
                .as_deref()
                .ok_or_eyre("user does not have username")?;
            println!("{bullet} {username}");
        }
    }

    Ok(())
}

async fn list_followers(api: &Forgejo, user: Option<&str>) -> eyre::Result<()> {
    let followers = match user {
        Some(user) => {
            let query = forgejo_api::structs::UserListFollowersQuery {
                limit: Some(u32::MAX),
                ..Default::default()
            };
            api.user_list_followers(user, query).await?
        }
        None => {
            let query = forgejo_api::structs::UserCurrentListFollowersQuery {
                limit: Some(u32::MAX),
                ..Default::default()
            };
            api.user_current_list_followers(query).await?
        }
    };

    if followers.is_empty() {
        match user {
            Some(name) => println!("{name} has no followers"),
            None => println!("You have no followers :("),
        }
    } else {
        match user {
            Some(name) => println!("{name} is followed by:"),
            None => println!("You are followed by:"),
        }
        let SpecialRender { bullet, .. } = *crate::special_render();

        for follower in followers {
            let username = follower
                .login
                .as_deref()
                .ok_or_eyre("user does not have username")?;
            println!("{bullet} {username}");
        }
    }

    Ok(())
}

async fn block_user(api: &Forgejo, user: &str) -> eyre::Result<()> {
    api.user_block_user(user).await?;
    println!("Blocked {user}");
    Ok(())
}

async fn unblock_user(api: &Forgejo, user: &str) -> eyre::Result<()> {
    api.user_unblock_user(user).await?;
    println!("Unblocked {user}");
    Ok(())
}

#[derive(clap::ValueEnum, Clone, Debug, Default)]
pub enum RepoSortOrder {
    #[default]
    Name,
    Modified,
    Created,
    Stars,
    Forks,
}

async fn list_repos(
    api: &Forgejo,
    user: Option<&str>,
    starred: bool,
    sort: Option<RepoSortOrder>,
) -> eyre::Result<()> {
    let mut repos = if starred {
        match user {
            Some(user) => {
                let query = forgejo_api::structs::UserListStarredQuery {
                    limit: Some(u32::MAX),
                    ..Default::default()
                };
                api.user_list_starred(user, query).await?
            }
            None => {
                let query = forgejo_api::structs::UserCurrentListStarredQuery {
                    limit: Some(u32::MAX),
                    ..Default::default()
                };
                api.user_current_list_starred(query).await?
            }
        }
    } else {
        match user {
            Some(user) => {
                let query = forgejo_api::structs::UserListReposQuery {
                    limit: Some(u32::MAX),
                    ..Default::default()
                };
                api.user_list_repos(user, query).await?
            }
            None => {
                let query = forgejo_api::structs::UserCurrentListReposQuery {
                    limit: Some(u32::MAX),
                    ..Default::default()
                };
                api.user_current_list_repos(query).await?
            }
        }
    };

    if repos.is_empty() {
        if starred {
            match user {
                Some(user) => println!("{user} has not starred any repos"),
                None => println!("You have not starred any repos"),
            }
        } else {
            match user {
                Some(user) => println!("{user} does not own any repos"),
                None => println!("You do not own any repos"),
            }
        };
    } else {
        let sort_fn: fn(
            &forgejo_api::structs::Repository,
            &forgejo_api::structs::Repository,
        ) -> std::cmp::Ordering = match sort.unwrap_or_default() {
            RepoSortOrder::Name => |a, b| a.full_name.cmp(&b.full_name),
            RepoSortOrder::Modified => |a, b| b.updated_at.cmp(&a.updated_at),
            RepoSortOrder::Created => |a, b| b.created_at.cmp(&a.created_at),
            RepoSortOrder::Stars => |a, b| b.stars_count.cmp(&a.stars_count),
            RepoSortOrder::Forks => |a, b| b.forks_count.cmp(&a.forks_count),
        };
        repos.sort_unstable_by(sort_fn);

        let SpecialRender { bullet, .. } = *crate::special_render();
        for repo in &repos {
            let name = repo
                .full_name
                .as_deref()
                .ok_or_eyre("repo does not have name")?;
            println!("{bullet} {name}");
        }
        if repos.len() == 1 {
            println!("1 repo");
        } else {
            println!("{} repos", repos.len());
        }
    }

    Ok(())
}

async fn list_orgs(api: &Forgejo, user: Option<&str>) -> eyre::Result<()> {
    let mut orgs = match user {
        Some(user) => {
            let query = forgejo_api::structs::OrgListUserOrgsQuery {
                limit: Some(u32::MAX),
                ..Default::default()
            };
            api.org_list_user_orgs(user, query).await?
        }
        None => {
            let query = forgejo_api::structs::OrgListCurrentUserOrgsQuery {
                limit: Some(u32::MAX),
                ..Default::default()
            };
            api.org_list_current_user_orgs(query).await?
        }
    };

    if orgs.is_empty() {
        match user {
            Some(user) => println!("{user} is not a member of any organizations"),
            None => println!("You are not a member of any organizations"),
        }
    } else {
        orgs.sort_unstable_by(|a, b| a.name.cmp(&b.name));

        let SpecialRender { bullet, dash, .. } = *crate::special_render();
        for org in &orgs {
            let name = org.name.as_deref().ok_or_eyre("org does not have name")?;
            let full_name = org
                .full_name
                .as_deref()
                .ok_or_eyre("org does not have name")?;
            if !full_name.is_empty() {
                println!("{bullet} {name} {dash} \"{full_name}\"");
            } else {
                println!("{bullet} {name}");
            }
        }
        if orgs.len() == 1 {
            println!("1 organization");
        } else {
            println!("{} organizations", orgs.len());
        }
    }
    Ok(())
}

async fn list_activity(api: &Forgejo, user: Option<&str>) -> eyre::Result<()> {
    let user = match user {
        Some(s) => s.to_owned(),
        None => {
            let myself = api.user_get_current().await?;
            myself.login.ok_or_eyre("current user does not have name")?
        }
    };
    let query = forgejo_api::structs::UserListActivityFeedsQuery {
        only_performed_by: Some(true),
        ..Default::default()
    };
    let feed = api.user_list_activity_feeds(&user, query).await?;

    let SpecialRender {
        bold,
        yellow,
        bright_cyan,
        reset,
        ..
    } = *crate::special_render();

    for activity in feed {
        let actor = activity
            .act_user
            .as_ref()
            .ok_or_eyre("activity does not have actor")?;
        let actor_name = actor
            .login
            .as_deref()
            .ok_or_eyre("actor does not have name")?;
        let op_type = activity
            .op_type
            .as_ref()
            .ok_or_eyre("activity does not have op type")?;

        // do not add ? to these. they are here to make each branch smaller
        let repo = activity
            .repo
            .as_ref()
            .ok_or_eyre("activity does not have repo");
        let content = activity
            .content
            .as_deref()
            .ok_or_eyre("activity does not have content");
        let ref_name = activity
            .ref_name
            .as_deref()
            .ok_or_eyre("repo does not have full name");

        fn issue_name<'a, 'b>(
            repo: &'a forgejo_api::structs::Repository,
            content: &'b str,
        ) -> eyre::Result<(&'a str, &'b str)> {
            let full_name = repo
                .full_name
                .as_deref()
                .ok_or_eyre("repo does not have full name")?;
            let (issue_id, _issue_name) = content.split_once("|").unwrap_or((content, ""));
            Ok((full_name, issue_id))
        }

        print!("");
        use forgejo_api::structs::ActivityOpType;
        match op_type {
            ActivityOpType::CreateRepo => {
                let repo = repo?;
                let full_name = repo
                    .full_name
                    .as_deref()
                    .ok_or_eyre("repo does not have full name")?;
                if let Some(parent) = &repo.parent {
                    let parent_full_name = parent
                        .full_name
                        .as_deref()
                        .ok_or_eyre("parent repo does not have full name")?;
                    println!("{bold}{actor_name}{reset} forked repository {bold}{yellow}{parent_full_name}{reset} to {bold}{yellow}{full_name}{reset}");
                } else {
                    if repo.mirror.is_some_and(|b| b) {
                        println!("{bold}{actor_name}{reset} created mirror {bold}{yellow}{full_name}{reset}");
                    } else {
                        println!("{bold}{actor_name}{reset} created repository {bold}{yellow}{full_name}{reset}");
                    }
                }
            }
            ActivityOpType::RenameRepo => {
                let repo = repo?;
                let content = content?;
                let full_name = repo
                    .full_name
                    .as_deref()
                    .ok_or_eyre("repo does not have full name")?;
                println!("{bold}{actor_name}{reset} renamed repository from {bold}{yellow}\"{content}\"{reset} to {bold}{yellow}{full_name}{reset}");
            }
            ActivityOpType::StarRepo => {
                let repo = repo?;
                let full_name = repo
                    .full_name
                    .as_deref()
                    .ok_or_eyre("repo does not have full name")?;
                println!(
                    "{bold}{actor_name}{reset} starred repository {bold}{yellow}{full_name}{reset}"
                );
            }
            ActivityOpType::WatchRepo => {
                let repo = repo?;
                let full_name = repo
                    .full_name
                    .as_deref()
                    .ok_or_eyre("repo does not have full name")?;
                println!(
                    "{bold}{actor_name}{reset} watched repository {bold}{yellow}{full_name}{reset}"
                );
            }
            ActivityOpType::CommitRepo => {
                let repo = repo?;
                let full_name = repo
                    .full_name
                    .as_deref()
                    .ok_or_eyre("repo does not have full name")?;
                let ref_name = ref_name?;
                let branch = ref_name
                    .rsplit_once("/")
                    .map(|(_, b)| b)
                    .unwrap_or(ref_name);
                if !content?.is_empty() {
                    println!("{bold}{actor_name}{reset} pushed to {bold}{bright_cyan}{branch}{reset} on {bold}{yellow}{full_name}{reset}");
                }
            }
            ActivityOpType::CreateIssue => {
                let (name, id) = issue_name(repo?, content?)?;
                println!("{bold}{actor_name}{reset} opened issue {bold}{yellow}{name}#{id}{reset}");
            }
            ActivityOpType::CreatePullRequest => {
                let (name, id) = issue_name(repo?, content?)?;
                println!("{bold}{actor_name}{reset} created pull request {bold}{yellow}{name}#{id}{reset}");
            }
            ActivityOpType::TransferRepo => {
                let repo = repo?;
                let full_name = repo
                    .full_name
                    .as_deref()
                    .ok_or_eyre("repo does not have full name")?;
                let content = content?;
                println!("{bold}{actor_name}{reset} transfered repository {bold}{yellow}{content}{reset} to {bold}{yellow}{full_name}{reset}");
            }
            ActivityOpType::PushTag => {
                let repo = repo?;
                let full_name = repo
                    .full_name
                    .as_deref()
                    .ok_or_eyre("repo does not have full name")?;
                let ref_name = ref_name?;
                let tag = ref_name
                    .rsplit_once("/")
                    .map(|(_, b)| b)
                    .unwrap_or(ref_name);
                println!("{bold}{actor_name}{reset} pushed tag {bold}{bright_cyan}{tag}{reset} to {bold}{yellow}{full_name}{reset}");
            }
            ActivityOpType::CommentIssue => {
                let (name, id) = issue_name(repo?, content?)?;
                println!(
                    "{bold}{actor_name}{reset} commented on issue {bold}{yellow}{name}#{id}{reset}"
                );
            }
            ActivityOpType::MergePullRequest | ActivityOpType::AutoMergePullRequest => {
                let (name, id) = issue_name(repo?, content?)?;
                println!("{bold}{actor_name}{reset} merged pull request {bold}{yellow}{name}#{id}{reset}");
            }
            ActivityOpType::CloseIssue => {
                let (name, id) = issue_name(repo?, content?)?;
                println!("{bold}{actor_name}{reset} closed issue {bold}{yellow}{name}#{id}{reset}");
            }
            ActivityOpType::ReopenIssue => {
                let (name, id) = issue_name(repo?, content?)?;
                println!(
                    "{bold}{actor_name}{reset} reopened issue {bold}{yellow}{name}#{id}{reset}"
                );
            }
            ActivityOpType::ClosePullRequest => {
                let (name, id) = issue_name(repo?, content?)?;
                println!("{bold}{actor_name}{reset} closed pull request {bold}{yellow}{name}#{id}{reset}");
            }
            ActivityOpType::ReopenPullRequest => {
                let (name, id) = issue_name(repo?, content?)?;
                println!("{bold}{actor_name}{reset} reopened pull request {bold}{yellow}{name}#{id}{reset}");
            }
            ActivityOpType::DeleteTag => {
                let repo = repo?;
                let full_name = repo
                    .full_name
                    .as_deref()
                    .ok_or_eyre("repo does not have full name")?;
                let ref_name = ref_name?;
                let tag = ref_name
                    .rsplit_once("/")
                    .map(|(_, b)| b)
                    .unwrap_or(ref_name);
                println!("{bold}{actor_name}{reset} deleted tag {bold}{bright_cyan}{tag}{reset} from {bold}{yellow}{full_name}{reset}");
            }
            ActivityOpType::DeleteBranch => {
                let repo = repo?;
                let full_name = repo
                    .full_name
                    .as_deref()
                    .ok_or_eyre("repo does not have full name")?;
                let ref_name = ref_name?;
                let branch = ref_name
                    .rsplit_once("/")
                    .map(|(_, b)| b)
                    .unwrap_or(ref_name);
                println!("{bold}{actor_name}{reset} deleted branch {bold}{bright_cyan}{branch}{reset} from {bold}{yellow}{full_name}{reset}");
            }
            ActivityOpType::MirrorSyncPush => {}
            ActivityOpType::MirrorSyncCreate => {}
            ActivityOpType::MirrorSyncDelete => {}
            ActivityOpType::ApprovePullRequest => {
                let (name, id) = issue_name(repo?, content?)?;
                println!("{bold}{actor_name}{reset} approved {bold}{yellow}{name}#{id}{reset}");
            }
            ActivityOpType::RejectPullRequest => {
                let (name, id) = issue_name(repo?, content?)?;
                println!("{bold}{actor_name}{reset} suggested changes for {bold}{yellow}{name}#{id}{reset}");
            }
            ActivityOpType::CommentPull => {
                let (name, id) = issue_name(repo?, content?)?;
                println!("{bold}{actor_name}{reset} commented on pull request {bold}{yellow}{name}#{id}{reset}");
            }
            ActivityOpType::PublishRelease => {
                let repo = repo?;
                let full_name = repo
                    .full_name
                    .as_deref()
                    .ok_or_eyre("repo does not have full name")?;
                let content = content?;
                println!("{bold}{actor_name}{reset} created release {bold}{bright_cyan}\"{content}\"{reset} to {bold}{yellow}{full_name}{reset}");
            }
            ActivityOpType::PullReviewDismissed => {}
            ActivityOpType::PullRequestReadyForReview => {}
        }
    }
    Ok(())
}

fn default_settings_opt() -> forgejo_api::structs::UserSettingsOptions {
    forgejo_api::structs::UserSettingsOptions {
        description: None,
        diff_view_style: None,
        enable_repo_unit_hints: None,
        full_name: None,
        hide_activity: None,
        hide_email: None,
        language: None,
        location: None,
        pronouns: None,
        theme: None,
        website: None,
    }
}

async fn edit_bio(api: &Forgejo, new_bio: Option<String>) -> eyre::Result<()> {
    let new_bio = match new_bio {
        Some(s) => s,
        None => {
            let mut bio = api
                .user_get_current()
                .await?
                .description
                .unwrap_or_default();
            crate::editor(&mut bio, Some("md")).await?;
            bio
        }
    };
    let opt = forgejo_api::structs::UserSettingsOptions {
        description: Some(new_bio),
        ..default_settings_opt()
    };
    api.update_user_settings(opt).await?;
    Ok(())
}

async fn edit_name(api: &Forgejo, new_name: Option<String>, unset: bool) -> eyre::Result<()> {
    match (new_name, unset) {
        (Some(_), true) => unreachable!(),
        (Some(name), false) if !name.is_empty() => {
            let opt = forgejo_api::structs::UserSettingsOptions {
                full_name: Some(name),
                ..default_settings_opt()
            };
            api.update_user_settings(opt).await?;
        }
        (None, true) => {
            let opt = forgejo_api::structs::UserSettingsOptions {
                full_name: Some(String::new()),
                ..default_settings_opt()
            };
            api.update_user_settings(opt).await?;
        }
        _ => println!("Use --unset to remove your name from your profile"),
    }
    Ok(())
}

async fn edit_pronouns(
    api: &Forgejo,
    new_pronouns: Option<String>,
    unset: bool,
) -> eyre::Result<()> {
    match (new_pronouns, unset) {
        (Some(_), true) => unreachable!(),
        (Some(pronouns), false) if !pronouns.is_empty() => {
            let opt = forgejo_api::structs::UserSettingsOptions {
                pronouns: Some(pronouns),
                ..default_settings_opt()
            };
            api.update_user_settings(opt).await?;
        }
        (None, true) => {
            let opt = forgejo_api::structs::UserSettingsOptions {
                pronouns: Some(String::new()),
                ..default_settings_opt()
            };
            api.update_user_settings(opt).await?;
        }
        _ => println!("Use --unset to remove your pronouns from your profile"),
    }
    Ok(())
}

async fn edit_location(
    api: &Forgejo,
    new_location: Option<String>,
    unset: bool,
) -> eyre::Result<()> {
    match (new_location, unset) {
        (Some(_), true) => unreachable!(),
        (Some(location), false) if !location.is_empty() => {
            let opt = forgejo_api::structs::UserSettingsOptions {
                location: Some(location),
                ..default_settings_opt()
            };
            api.update_user_settings(opt).await?;
        }
        (None, true) => {
            let opt = forgejo_api::structs::UserSettingsOptions {
                location: Some(String::new()),
                ..default_settings_opt()
            };
            api.update_user_settings(opt).await?;
        }
        _ => println!("Use --unset to remove your location from your profile"),
    }
    Ok(())
}

async fn edit_activity(api: &Forgejo, visibility: VisbilitySetting) -> eyre::Result<()> {
    let opt = forgejo_api::structs::UserSettingsOptions {
        hide_activity: Some(visibility == VisbilitySetting::Hidden),
        ..default_settings_opt()
    };
    api.update_user_settings(opt).await?;
    Ok(())
}

async fn edit_email(
    api: &Forgejo,
    visibility: Option<VisbilitySetting>,
    add: Vec<String>,
    rm: Vec<String>,
) -> eyre::Result<()> {
    if let Some(vis) = visibility {
        let opt = forgejo_api::structs::UserSettingsOptions {
            hide_activity: Some(vis == VisbilitySetting::Hidden),
            ..default_settings_opt()
        };
        api.update_user_settings(opt).await?;
    }
    if !add.is_empty() {
        let opt = forgejo_api::structs::CreateEmailOption { emails: Some(add) };
        api.user_add_email(opt).await?;
    }
    if !rm.is_empty() {
        let opt = forgejo_api::structs::DeleteEmailOption { emails: Some(rm) };
        api.user_delete_email(opt).await?;
    }
    Ok(())
}

async fn edit_website(api: &Forgejo, new_url: Option<String>, unset: bool) -> eyre::Result<()> {
    match (new_url, unset) {
        (Some(_), true) => unreachable!(),
        (Some(url), false) if !url.is_empty() => {
            let opt = forgejo_api::structs::UserSettingsOptions {
                website: Some(url),
                ..default_settings_opt()
            };
            api.update_user_settings(opt).await?;
        }
        (None, true) => {
            let opt = forgejo_api::structs::UserSettingsOptions {
                website: Some(String::new()),
                ..default_settings_opt()
            };
            api.update_user_settings(opt).await?;
        }
        _ => println!("Use --unset to remove your name from your profile"),
    }
    Ok(())
}
