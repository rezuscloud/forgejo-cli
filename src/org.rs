use clap::{Args, Subcommand};
use eyre::OptionExt;
use forgejo_api::{structs::CreateOrgOption, Forgejo};

use crate::{
    repo::{RepoInfo, RepoName},
    SpecialRender,
};

#[derive(Args, Clone, Debug)]
pub struct OrgCommand {
    /// The local git remote that points to the repo to operate on.
    #[clap(long, short = 'R')]
    remote: Option<String>,
    #[clap(subcommand)]
    command: OrgSubcommand,
}

#[derive(Subcommand, Clone, Debug)]
pub enum OrgSubcommand {
    Create {
        /// The username for the organization.
        ///
        /// It can only have alphanumeric characters, dash, underscore, or period. It must start
        /// and end with an alphanumeric character, and can't have consecutive dashes, underscores,
        /// or periods.
        ///
        /// If you want a name that doesn't have these restrictions, see the `--full-name` option.
        name: String,
        /// The display name for the organization.
        ///
        /// This doesn't have the restrictions the `name` argument does, and can contain any UTF-8
        /// text.
        #[clap(long, short)]
        full_name: Option<String>,
        /// The organization's description
        #[clap(long, short)]
        description: Option<String>,
        /// Contact email for the organization
        #[clap(long, short)]
        email: Option<String>,
        /// The organizations's location
        #[clap(long, short)]
        location: Option<String>,
        /// The organization's website
        #[clap(long, short)]
        website: Option<String>,
        /// The visibility of the organization.
        ///
        /// Public organizations can be viewed by anyone, limited orgs can only be viewed by
        /// logged-in users, and private orgs can only be viewed by members of that org.
        #[clap(long, short)]
        visibility: Option<OrgVisibility>,
        /// Whether the admin of a repo can change org teams' access to it.
        #[clap(long, short)]
        admin_can_change_team_access: bool,
    },
}

#[derive(clap::ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrgVisibility {
    Private,
    Limited,
    Public,
}

impl Into<forgejo_api::structs::CreateOrgOptionVisibility> for OrgVisibility {
    fn into(self) -> forgejo_api::structs::CreateOrgOptionVisibility {
        use forgejo_api::structs::CreateOrgOptionVisibility as ApiVis;
        match self {
            OrgVisibility::Private => ApiVis::Private,
            OrgVisibility::Limited => ApiVis::Limited,
            OrgVisibility::Public => ApiVis::Public,
        }
    }
}

impl OrgCommand {
    pub async fn run(self, keys: &mut crate::KeyInfo, host_name: Option<&str>) -> eyre::Result<()> {
        let repo = RepoInfo::get_current(host_name, None, self.remote.as_deref(), &keys)?;
        let api = keys.get_api(repo.host_url()).await?;
        match self.command {
            OrgSubcommand::Create {
                name,
                description,
                email,
                full_name,
                location,
                website,
                visibility,
                admin_can_change_team_access,
            } => {
                create_org(
                    &api,
                    name,
                    description,
                    email,
                    full_name,
                    location,
                    website,
                    visibility,
                    admin_can_change_team_access,
                )
                .await?
            }
        }
        Ok(())
    }
}

fn is_valid_name_char(c: char) -> bool {
    match c {
        '-' | '_' | '.' => true,
        _ => c.is_ascii_alphanumeric(),
    }
}

async fn create_org(
    api: &Forgejo,
    name: String,
    description: Option<String>,
    email: Option<String>,
    full_name: Option<String>,
    location: Option<String>,
    website: Option<String>,
    visibility: Option<OrgVisibility>,
    admin_can_change_team_access: bool,
) -> eyre::Result<()> {
    if !name.chars().all(is_valid_name_char) {
        eyre::bail!("Organization names can only have alphanumeric characters, dash, underscore, or period. \n  If you want a name with other characters, try setting the --full-name flag");
    }
    if !name
        .chars()
        .next()
        .is_some_and(|c| c.is_ascii_alphanumeric())
    {
        eyre::bail!("Organization names can only start with alphanumeric characters. \n  If you want a name that starts with other characters, try setting the --full-name flag");
    }
    if !name
        .chars()
        .last()
        .is_some_and(|c| c.is_ascii_alphanumeric())
    {
        eyre::bail!("Organization names can only end with alphanumeric characters. \n  If you want a name that ends with other characters, try setting the --full-name flag");
    }
    let mut chars = name.chars().peekable();
    while let Some(c) = chars.next() {
        // because of the prior check, if it isn't alphanumeric, it's definitely one of - _ or .
        if !c.is_alphanumeric() && !chars.peek().is_some_and(|c| c.is_alphanumeric()) {
            eyre::bail!("Organization names can't have consecutive non-alphanumberic characters.\n  If you want that in the name, try setting the --full-name flag");
        }
    }
    let opt = CreateOrgOption {
        description,
        email,
        full_name,
        location,
        repo_admin_change_team_access: Some(admin_can_change_team_access),
        username: name,
        visibility: visibility.map(|v| v.into()),
        website,
    };
    let new_org = api.org_create(opt).await?;

    let name = new_org.name.ok_or_eyre("new org does not have name")?;
    let visibility = new_org
        .visibility
        .ok_or_eyre("new org does not have visibility")?;

    let SpecialRender {
        fancy,
        bold,
        light_grey,
        reset,
        ..
    } = *crate::special_render();
    print!("created new {visibility} org ");
    if let Some(full_name) = &new_org.full_name {
        if fancy {
            println!("{bold}{full_name}{reset} {light_grey}({name}){reset}");
        } else {
            println!("\"{full_name}\" ({name})");
        }
    } else {
        if fancy {
            println!("{bold}{name}{reset}");
        } else {
            println!("\"{name}\"");
        }
    }
    Ok(())
}
