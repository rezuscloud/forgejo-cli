use std::{collections::BTreeMap, io::Write};

use clap::{Args, Subcommand};
use eyre::OptionExt;
use forgejo_api::{
    structs::{
        CreateOrgOption, CreateTeamOption, EditOrgOption, EditTeamOption, OrgGetAllQuery,
        OrgListCurrentUserOrgsQuery, OrgListTeamMembersQuery, OrgListTeamsQuery, User,
    },
    Forgejo,
};

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
    List {
        /// Which page of the results to view
        #[clap(long, short)]
        page: Option<u32>,
        /// Only list organizations you are a member of.
        #[clap(long, short)]
        only_member_of: bool,
    },
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
    View {
        /// The name of the organization to view.
        name: String,
    },
    Edit {
        /// The name of the organization to edit.
        ///
        /// Note that this is the username, *not* the display name.
        name: String,
        /// The display name for the organization.
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
    Activity {
        /// The name of the organization to view activity for.
        name: String,
    },
    #[clap(subcommand)]
    Team(TeamSubcommand),
}

#[derive(Subcommand, Clone, Debug)]
pub enum TeamSubcommand {
    List {
        /// The name of the organization to list the teams in.
        org: String,
    },
    Create {
        /// The name of the organization to create the team in.
        org: String,
        /// The name of the new team
        ///
        /// This must only contain alphanumeric characters.
        name: String,
        #[clap(long, short)]
        can_create_repos: bool,
        #[clap(long, short)]
        description: Option<String>,
        #[clap(long, short)]
        include_all_repos: bool,
        #[clap(long, short)]
        read_permissions: Option<String>,
        #[clap(long, short)]
        write_permissions: Option<String>,
        #[clap(long, short = 'A')]
        admin: bool,
    },
    View {
        /// The name of the organization the team is part of.
        org: String,
        /// The name of the new team
        name: String,
        #[clap(long, short = 'p')]
        list_permissions: bool,
        #[clap(long, short = 'm')]
        list_members: bool,
    },
    Edit {
        /// The name of the organization the team is in.
        org: String,
        /// The name of the team to edit
        name: String,
        #[clap(long, short)]
        new_name: Option<String>,
        #[clap(long, short)]
        can_create_repos: bool,
        #[clap(long, short)]
        description: Option<String>,
        #[clap(long, short)]
        include_all_repos: bool,
        #[clap(long, short)]
        read_permissions: Option<String>,
        #[clap(long, short)]
        write_permissions: Option<String>,
        #[clap(long, short = 'A')]
        admin: bool,
    },
    Delete {
        /// The name of the organization the team is in.
        org: String,
        /// The name of the team to delete
        name: String,
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

impl Into<forgejo_api::structs::EditOrgOptionVisibility> for OrgVisibility {
    fn into(self) -> forgejo_api::structs::EditOrgOptionVisibility {
        use forgejo_api::structs::EditOrgOptionVisibility as ApiVis;
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
            OrgSubcommand::List {
                page,
                only_member_of,
            } => list_orgs(&api, page, only_member_of).await?,
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
            OrgSubcommand::View { name } => view_org(&api, name).await?,
            OrgSubcommand::Edit {
                name,
                description,
                email,
                full_name,
                location,
                website,
                visibility,
                admin_can_change_team_access,
            } => {
                edit_org(
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
            OrgSubcommand::Activity { name } => list_activity(&api, name).await?,
            OrgSubcommand::Team(subcommand) => match subcommand {
                TeamSubcommand::List { org } => list_teams(&api, org).await?,
                TeamSubcommand::Create {
                    org,
                    name,
                    can_create_repos,
                    description,
                    include_all_repos,
                    read_permissions,
                    write_permissions,
                    admin,
                } => {
                    create_team(
                        &api,
                        org,
                        name,
                        can_create_repos,
                        description,
                        include_all_repos,
                        read_permissions,
                        write_permissions,
                        admin,
                    )
                    .await?
                }
                TeamSubcommand::View {
                    org,
                    name,
                    list_permissions,
                    list_members,
                } => view_team(&api, org, name, list_permissions, list_members).await?,
                TeamSubcommand::Edit {
                    org,
                    name,
                    new_name,
                    can_create_repos,
                    description,
                    include_all_repos,
                    read_permissions,
                    write_permissions,
                    admin,
                } => {
                    edit_team(
                        &api,
                        org,
                        name,
                        new_name,
                        can_create_repos,
                        description,
                        include_all_repos,
                        read_permissions,
                        write_permissions,
                        admin,
                    )
                    .await?
                }
                TeamSubcommand::Delete { org, name } => delete_team(&api, org, name).await?,
            },
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

async fn list_orgs(api: &Forgejo, page: Option<u32>, only_member_of: bool) -> eyre::Result<()> {
    let orgs = if only_member_of {
        let query = OrgListCurrentUserOrgsQuery { page, limit: None };
        let (_, orgs) = api.org_list_current_user_orgs(query).await?;
        orgs
    } else {
        let query = OrgGetAllQuery { page, limit: None };
        let (_, orgs) = api.org_get_all(query).await?;
        orgs
    };

    if orgs.is_empty() {
        println!("No results");
    } else {
        let SpecialRender {
            bullet,
            bold,
            reset,
            ..
        } = *crate::special_render();
        for org in orgs {
            let name = org.name.ok_or_eyre("org does not have name")?;
            println!("{bullet} {bold}{name}{reset}");
        }
    }
    Ok(())
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

async fn view_org(api: &Forgejo, name: String) -> eyre::Result<()> {
    let org = api.org_get(&name).await?;

    let SpecialRender {
        bold,
        dash,
        bright_cyan,
        light_grey,
        reset,
        ..
    } = *crate::special_render();

    let name = org.name.as_deref().ok_or_eyre("org does not have name")?;
    let visibility = org
        .visibility
        .as_deref()
        .ok_or_eyre("new org does not have visibility")?;
    let vis_pretty = match visibility {
        "public" => "Public",
        "limited" => "Limited",
        "private" => "Private",
        _ => visibility,
    };

    if let Some(full_name) = &org.full_name {
        print!("{bold}{bright_cyan}{full_name}{reset} {light_grey}({name}){reset}");
    } else {
        print!("{bold}{bright_cyan}{name}{reset}");
    }
    print!(" {dash} {vis_pretty}");
    println!();

    // This needs the x-total-count header name fixed in forgejo_api before it can work
    // let members_query = forgejo_api::structs::OrgListPublicMembersQuery {
    //     page: Some(1),
    //     limit: Some(1),
    // };
    // let (members_headers, _) = api.org_list_public_members(&name, members_query).await?;
    // let members = members_headers.x_total.unwrap_or_default();
    // let teams_query = forgejo_api::structs::OrgListTeamsQuery {
    //     page: Some(1),
    //     limit: Some(1),
    // };
    // let (teams_headers, _) = api.org_list_teams(&name, teams_query).await?;
    // let teams = teams_headers.x_total.unwrap_or_default();
    // println!("{bold}{members}{reset} members {dash} {bold}{teams}{reset} teams");

    let mut first = true;
    if let Some(website) = &org.website {
        if !website.is_empty() {
            print!("{bold}{website}{reset}");
            first = false;
        }
    }
    if let Some(email) = &org.email {
        if !email.is_empty() {
            if !first {
                print!(" {dash} ");
            }
            print!("{email}");
            first = false;
        }
    }
    if let Some(location) = &org.location {
        if !location.is_empty() {
            if !first {
                print!(" {dash} ");
            }
            print!("{location}");
            first = false;
        }
    }
    if !first {
        println!();
    }

    if let Some(description) = &org.description {
        if !description.is_empty() {
            println!("\n{}\n", crate::markdown(&description));
        }
    }

    Ok(())
}

async fn edit_org(
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
    let opt = EditOrgOption {
        description,
        email,
        full_name,
        location,
        repo_admin_change_team_access: Some(admin_can_change_team_access),
        visibility: visibility.map(|v| v.into()),
        website,
    };
    api.org_edit(&name, opt).await?;
    Ok(())
}

async fn list_activity(api: &Forgejo, name: String) -> eyre::Result<()> {
    let query = forgejo_api::structs::OrgListActivityFeedsQuery::default();
    let (_, feed) = api.org_list_activity_feeds(&name, query).await?;

    for activity in feed {
        crate::user::print_activity(&activity)?;
    }
    Ok(())
}

async fn find_team_by_name(
    api: &Forgejo,
    org: &str,
    name: &str,
) -> eyre::Result<forgejo_api::structs::Team> {
    let mut seen = 0;
    for page in 1.. {
        let query = OrgListTeamsQuery {
            page: Some(page),
            limit: None,
        };
        let (headers, teams) = api.org_list_teams(&org, query).await?;
        seen += teams.len();
        for team in teams {
            if team
                .name
                .as_deref()
                .is_some_and(|team_name| team_name == name)
            {
                return Ok(team);
            }
        }
        if seen >= headers.x_total_count.unwrap_or_default() as usize {
            break;
        }
    }
    eyre::bail!("Unknown team {name}");
}

async fn list_teams(api: &Forgejo, org: String) -> eyre::Result<()> {
    let mut teams = Vec::new();
    for page_idx in 1.. {
        let query = OrgListTeamsQuery {
            page: Some(page_idx),
            limit: None,
        };
        let (headers, page) = api.org_list_teams(&org, query).await?;
        teams.extend(page);
        if teams.len() >= headers.x_total_count.unwrap_or_default() as usize {
            break;
        }
    }
    teams.sort_unstable_by_key(permission_sort_id);

    let SpecialRender {
        bright_blue,
        bold,
        reset,
        bullet,
        ..
    } = crate::special_render();
    for team in teams {
        let team_name = team.name.as_deref().ok_or_eyre("team does not have name")?;
        println!("{bullet} {bold}{bright_blue}{team_name}{reset}");
    }
    Ok(())
}

fn permission_sort_id(team: &forgejo_api::structs::Team) -> u32 {
    use forgejo_api::structs::TeamPermission as Perm;
    match &team.permission {
        Some(Perm::Owner) => 0,
        Some(Perm::Admin) => 1,
        Some(Perm::Write) => 2,
        Some(Perm::Read) => 3,
        Some(Perm::None) | None => 4,
    }
}

const ALL_UNITS: &[&str] = &[
    "repo.wiki",
    "repo.ext_wiki",
    "repo.issues",
    "repo.ext_issues",
    "repo.pulls",
    "repo.projects",
    "repo.actions",
    "repo.code",
    "repo.releases",
    "repo.packages",
];

fn create_unit_map(ro_perms: Option<&str>, rw_perms: Option<&str>) -> BTreeMap<String, String> {
    let mut units = BTreeMap::new();
    if let Some(ro_perms) = ro_perms {
        if ro_perms == "all" {
            for ro in ALL_UNITS {
                units.insert(ro.to_string(), "read".to_owned());
            }
        } else {
            for ro in ro_perms.split(",") {
                units.insert(format!("repo.{ro}"), "read".to_owned());
            }
        }
    }
    if let Some(rw_perms) = rw_perms {
        if rw_perms.trim() == "all" {
            for rw in ALL_UNITS {
                units.insert(rw.to_string(), "write".to_owned());
            }
        } else {
            for rw in rw_perms.split(",") {
                units.insert(format!("repo.{rw}"), "write".to_owned());
            }
        }
    }
    units
}

async fn create_team(
    api: &Forgejo,
    org: String,
    name: String,
    can_create_repo: bool,
    description: Option<String>,
    include_all_repos: bool,
    read_permissions: Option<String>,
    write_permissions: Option<String>,
    admin: bool,
) -> eyre::Result<()> {
    let units = create_unit_map(read_permissions.as_deref(), write_permissions.as_deref());
    let options = CreateTeamOption {
        can_create_org_repo: Some(can_create_repo),
        description,
        includes_all_repositories: Some(include_all_repos),
        name,
        permission: admin.then(|| forgejo_api::structs::CreateTeamOptionPermission::Admin),
        units: None,
        units_map: Some(units),
    };
    let new_team = api.org_create_team(&org, options).await?;
    let org = new_team.organization.ok_or_eyre("team doesn't have org")?;
    let org_name = org
        .name
        .or(org.full_name)
        .ok_or_eyre("org doesn't have name")?;
    let name = new_team.name.ok_or_eyre("team doesn't have name")?;

    let SpecialRender {
        bright_blue,
        bold,
        reset,
        ..
    } = crate::special_render();
    print!("created new ");
    if admin {
        print!("admin ");
    }
    println!("team {bright_blue}{bold}{name}{reset} in {bold}{org_name}{reset}");
    Ok(())
}

async fn view_team(
    api: &Forgejo,
    org: String,
    name: String,
    list_permissions: bool,
    list_members: bool,
) -> eyre::Result<()> {
    let team = find_team_by_name(api, &org, &name).await?;

    let SpecialRender {
        bright_blue,
        bright_red,
        bold,
        reset,
        dash,
        ..
    } = crate::special_render();

    print!("{bright_blue}{bold}{name}{reset} {dash} in org {bold}{org}{reset}");
    if team
        .permission
        .is_some_and(|p| p == forgejo_api::structs::TeamPermission::Admin)
    {
        print!(" {dash} {bright_red}Admin{reset}");
    }
    println!();

    if let Some(description) = &team.description {
        if !description.is_empty() {
            println!("\n{}", crate::markdown(description));
        }
    }

    if list_permissions {
        println!();
        let units = team
            .units_map
            .as_ref()
            .ok_or_eyre("team does not have permission units")?;
        let mut ro_perms = Vec::new();
        let mut rw_perms = Vec::new();
        for (unit, permission) in units {
            match &**permission {
                "read" => ro_perms.push(unit),
                "write" | "admin" | "owner" => rw_perms.push(unit),
                _ => (),
            }
        }

        let get_unit_name = |unit| match unit {
            "repo.wiki" => "Wikis",
            "repo.ext_wiki" => "External Wikis",
            "repo.issues" => "Issues",
            "repo.ext_issues" => "External Issues",
            "repo.pulls" => "Pull Requests",
            "repo.projects" => "Projects",
            "repo.actions" => "CI",
            "repo.code" => "Code",
            "repo.releases" => "Releases",
            "repo.packages" => "Packages",
            _ => "Unknown",
        };
        if !ro_perms.is_empty() {
            print!("Read Only: ");
            for (i, unit) in ro_perms.iter().enumerate() {
                let unit_name = get_unit_name(unit);
                if i > 0 {
                    print!(", ");
                }
                print!("{unit_name}");
            }
            println!();
        }
        if !rw_perms.is_empty() {
            print!("Read/Write: ");
            for (i, unit) in rw_perms.iter().enumerate() {
                let unit_name = get_unit_name(unit);
                if i != 0 {
                    print!(", ");
                }
                print!("{unit_name}");
            }
            println!();
        }
    }

    if list_members {
        let team_id = team.id.ok_or_eyre("team does not have id")?;
        println!();
        print!("Loading members...");
        std::io::stdout().flush()?;
        let mut members = Vec::new();
        for page_idx in 1.. {
            let query = OrgListTeamMembersQuery {
                page: Some(page_idx),
                limit: None,
            };
            let (_, page) = api.org_list_team_members(team_id as u64, query).await?;
            if page.is_empty() {
                break;
            }

            members.extend(page);
        }
        members.sort_by(|a, b| a.login.cmp(&b.login));
        print!("\r                  \r");
        println!("{bold}Members:{reset}");
        let max_line_length = crate::max_line_length();
        let mut current_line_length = 0;
        for (i, member) in members.into_iter().enumerate() {
            let username = member
                .login
                .as_deref()
                .ok_or_eyre("user does not have name")?;
            if i > 0 {
                print!(", ");
            }
            if current_line_length > 0 && current_line_length + username.len() > max_line_length {
                println!();
                current_line_length = 0;
            }
            print!("{username}");
            current_line_length += username.len() + 2;
        }
    }

    Ok(())
}

async fn edit_team(
    api: &Forgejo,
    org: String,
    name: String,
    new_name: Option<String>,
    can_create_repo: bool,
    description: Option<String>,
    include_all_repos: bool,
    read_permissions: Option<String>,
    write_permissions: Option<String>,
    admin: bool,
) -> eyre::Result<()> {
    let team = find_team_by_name(api, &org, &name).await?;
    let id = team.id.ok_or_eyre("team does not have id")?;

    // EditTeamOption's team field is a String rather than Option<String>
    // That should be fixed, but this gets around it for now.
    let new_name = new_name.unwrap_or(name);
    let units = create_unit_map(read_permissions.as_deref(), write_permissions.as_deref());

    let options = EditTeamOption {
        can_create_org_repo: Some(can_create_repo),
        description,
        includes_all_repositories: Some(include_all_repos),
        name: new_name,
        permission: admin.then(|| forgejo_api::structs::EditTeamOptionPermission::Admin),
        units: None,
        units_map: Some(units),
    };
    api.org_edit_team(id as u32, options).await?;

    Ok(())
}

async fn delete_team(api: &Forgejo, org: String, name: String) -> eyre::Result<()> {
    let SpecialRender { bold, reset, .. } = crate::special_render();
    println!("Are you sure you want to delete {bold}{org}/{name}{reset}?");
    let confirmation = crate::readline("(y/N) ").await?.to_lowercase();
    if matches!(confirmation.trim(), "y" | "yes") {
        let id = find_team_by_name(api, &org, &name)
            .await?
            .id
            .ok_or_eyre("team does not have id")?;
        api.org_delete_team(id as u64).await?;
        println!("Team deleted.");
    } else {
        println!("Team not deleted.");
    }
    Ok(())
}
