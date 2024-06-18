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
