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
    View {
        /// The name of the user to view
        user: Option<String>,
    },
    Browse {
        /// The name of the user to open in your browser
        user: Option<String>,
    },
    Follow {
        /// The name of the user to follow
        user: String,
    },
    Unfollow {
        /// The name of the user to follow
        user: String,
    },
    Following {
        /// The name of the user whose follows to list
        user: Option<String>,
    },
    Followers {
        /// The name of the user whose followers to list
        user: Option<String>,
    },
    Block {
        /// The name of the user to block
        user: String,
    },
    Unblock {
        /// The name of the user to unblock
        user: String,
    },
}

impl UserCommand {
    pub async fn run(self, keys: &mut crate::KeyInfo, host_name: Option<&str>) -> eyre::Result<()> {
        let repo = RepoInfo::get_current(host_name, None, self.remote.as_deref())?;
        let api = keys.get_api(repo.host_url()).await?;
        match self.command {
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
        }
        Ok(())
    }
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
