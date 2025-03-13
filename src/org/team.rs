use std::{collections::BTreeMap, io::Write};

use clap::{Args, Subcommand};
use eyre::OptionExt;
use forgejo_api::{
    structs::{
        CreateTeamOption, EditTeamOption, OrgListTeamMembersQuery, OrgListTeamReposQuery,
        OrgListTeamsQuery,
    },
    Forgejo,
};

use crate::SpecialRender;

#[derive(Subcommand, Clone, Debug)]
pub enum TeamSubcommand {
    List {
        /// The name of the organization to list the teams in.
        org: String,
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
    Create {
        /// The name of the organization to create the team in.
        org: String,
        /// The name of the new team
        ///
        /// This must only contain alphanumeric characters.
        name: String,
        #[clap(flatten)]
        options: TeamOptions,
    },
    Edit {
        /// The name of the organization the team is in.
        org: String,
        /// The name of the team to edit
        name: String,
        #[clap(long, short)]
        new_name: Option<String>,
        #[clap(flatten)]
        options: TeamOptions,
    },
    Delete {
        /// The name of the organization the team is in.
        org: String,
        /// The name of the team to delete
        name: String,
    },
    #[clap(subcommand)]
    Repo(TeamRepoSubcommand),
    #[clap(subcommand)]
    Member(TeamMemberSubcommand),
}

#[derive(Args, Clone, Debug)]
pub struct TeamOptions {
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
}

impl TeamSubcommand {
    pub async fn run(self, api: &forgejo_api::Forgejo) -> eyre::Result<()> {
        match self {
            TeamSubcommand::List { org } => list_teams(&api, org).await?,
            TeamSubcommand::View {
                org,
                name,
                list_permissions,
                list_members,
            } => view_team(&api, org, name, list_permissions, list_members).await?,
            TeamSubcommand::Create { org, name, options } => {
                create_team(&api, org, name, options).await?
            }
            TeamSubcommand::Edit {
                org,
                name,
                new_name,
                options,
            } => edit_team(&api, org, name, new_name, options).await?,
            TeamSubcommand::Delete { org, name } => delete_team(&api, org, name).await?,
            TeamSubcommand::Repo(subcommand) => subcommand.run(&api).await?,
            TeamSubcommand::Member(subcommand) => subcommand.run(&api).await?,
        }
        Ok(())
    }
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
    options: TeamOptions,
) -> eyre::Result<()> {
    let units = create_unit_map(
        options.read_permissions.as_deref(),
        options.write_permissions.as_deref(),
    );
    let opt = CreateTeamOption {
        can_create_org_repo: Some(options.can_create_repos),
        description: options.description,
        includes_all_repositories: Some(options.include_all_repos),
        name,
        permission: options
            .admin
            .then(|| forgejo_api::structs::CreateTeamOptionPermission::Admin),
        units: None,
        units_map: Some(units),
    };
    let new_team = api.org_create_team(&org, opt).await?;
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
    if options.admin {
        print!("admin ");
    }
    println!("team {bright_blue}{bold}{name}{reset} in {bold}{org_name}{reset}");
    Ok(())
}

async fn edit_team(
    api: &Forgejo,
    org: String,
    name: String,
    new_name: Option<String>,
    options: TeamOptions,
) -> eyre::Result<()> {
    let team = find_team_by_name(api, &org, &name).await?;
    let id = team.id.ok_or_eyre("team does not have id")?;

    // EditTeamOption's team field is a String rather than Option<String>
    // That should be fixed, but this gets around it for now.
    let new_name = new_name.unwrap_or(name);
    let units = create_unit_map(
        options.read_permissions.as_deref(),
        options.write_permissions.as_deref(),
    );

    let options = EditTeamOption {
        can_create_org_repo: Some(options.can_create_repos),
        description: options.description,
        includes_all_repositories: Some(options.include_all_repos),
        name: new_name,
        permission: options
            .admin
            .then(|| forgejo_api::structs::EditTeamOptionPermission::Admin),
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

#[derive(Subcommand, Clone, Debug)]
pub enum TeamRepoSubcommand {
    List {
        /// The name of the organization the team is in.
        org: String,
        /// The name of the team to view the repos of.
        team: String,
        /// Which page of the results to view
        #[clap(long, short)]
        page: Option<u32>,
    },
    Add {
        /// The name of the organization the team is in.
        org: String,
        /// The name of the team to add a repo to.
        team: String,
        /// The name of the repo to add to the team.
        repo: String,
    },
    Rm {
        /// The name of the organization the team is in.
        org: String,
        /// The name of the team to remove the repo from.
        team: String,
        /// The name of the repo to remove from the team.
        repo: String,
    },
}

impl TeamRepoSubcommand {
    async fn run(self, api: &Forgejo) -> eyre::Result<()> {
        match self {
            TeamRepoSubcommand::List { org, team, page } => {
                list_team_repos(&api, org, team, page).await?
            }
            TeamRepoSubcommand::Add { org, team, repo } => {
                add_repo_to_team(&api, org, team, repo).await?
            }
            TeamRepoSubcommand::Rm { org, team, repo } => {
                remove_repo_from_team(&api, org, team, repo).await?
            }
        }
        Ok(())
    }
}

async fn list_team_repos(
    api: &Forgejo,
    org: String,
    team: String,
    page: Option<u32>,
) -> eyre::Result<()> {
    let id = find_team_by_name(api, &org, &team)
        .await?
        .id
        .ok_or_eyre("team does not have id")?;
    let query = OrgListTeamReposQuery {
        page,
        limit: Some(20),
    };
    let (headers, repos) = api.org_list_team_repos(id as u64, query).await?;

    let SpecialRender { bullet, .. } = crate::special_render();
    if repos.is_empty() {
        println!("No results");
    } else {
        for repo in repos {
            let full_name = repo
                .full_name
                .as_deref()
                .ok_or_eyre("repo does not have full name")?;
            println!("{bullet} {full_name}");
        }
        let count = headers.x_total_count.unwrap_or_default() as u64;
        println!("Page {} of {}", page.unwrap_or(1), count.div_ceil(20));
    }
    Ok(())
}

async fn add_repo_to_team(
    api: &Forgejo,
    org: String,
    team: String,
    repo: String,
) -> eyre::Result<()> {
    let id = find_team_by_name(api, &org, &team)
        .await?
        .id
        .ok_or_eyre("team does not have id")?;
    api.org_add_team_repository(id as u64, &org, &repo).await?;
    let SpecialRender {
        bold,
        reset,
        bright_blue,
        ..
    } = crate::special_render();
    println!("Added {bold}{org}/{repo}{reset} to team {bright_blue}{bold}{team}{reset}");
    Ok(())
}

async fn remove_repo_from_team(
    api: &Forgejo,
    org: String,
    team: String,
    repo: String,
) -> eyre::Result<()> {
    let id = find_team_by_name(api, &org, &team)
        .await?
        .id
        .ok_or_eyre("team does not have id")?;
    api.org_remove_team_repository(id as u64, &org, &repo)
        .await?;
    let SpecialRender {
        bold,
        reset,
        bright_blue,
        ..
    } = crate::special_render();
    println!("Removed {bold}{org}/{repo}{reset} from team {bright_blue}{bold}{team}{reset}");
    Ok(())
}

#[derive(Subcommand, Clone, Debug)]
pub enum TeamMemberSubcommand {
    List {
        /// The name of the organization the team is in.
        org: String,
        /// The name of the team to view the members of.
        team: String,
        /// Which page of the results to view
        #[clap(long, short)]
        page: Option<u32>,
    },
    Add {
        /// The name of the organization the team is in.
        org: String,
        /// The name of the team to add a user to.
        team: String,
        /// The name of the user to add to the team.
        user: String,
    },
    Rm {
        /// The name of the organization the team is in.
        org: String,
        /// The name of the team to remove the user from.
        team: String,
        /// The name of the user to remove from the team.
        user: String,
    },
}

impl TeamMemberSubcommand {
    async fn run(self, api: &Forgejo) -> eyre::Result<()> {
        match self {
            TeamMemberSubcommand::List { org, team, page } => {
                list_team_members(&api, org, team, page).await?
            }
            TeamMemberSubcommand::Add { org, team, user } => {
                add_user_to_team(&api, org, team, user).await?
            }
            TeamMemberSubcommand::Rm { org, team, user } => {
                remove_user_from_team(&api, org, team, user).await?
            }
        }
        Ok(())
    }
}

async fn list_team_members(
    api: &Forgejo,
    org: String,
    team: String,
    page: Option<u32>,
) -> eyre::Result<()> {
    let id = find_team_by_name(api, &org, &team)
        .await?
        .id
        .ok_or_eyre("team does not have id")?;
    let query = OrgListTeamMembersQuery {
        page,
        limit: Some(20),
    };
    let (headers, users) = api.org_list_team_members(id as u64, query).await?;

    let SpecialRender {
        bullet,
        light_grey,
        bright_cyan,
        reset,
        ..
    } = crate::special_render();
    if users.is_empty() {
        println!("No results");
    } else {
        for user in users {
            let username = user
                .login
                .as_deref()
                .ok_or_eyre("repo does not have full name")?;
            match user.full_name.as_deref().filter(|s| !s.is_empty()) {
                Some(full_name) => println!(
                    "{bullet} {bright_cyan}{full_name}{reset} {light_grey}({username}){reset}"
                ),
                None => println!("{bullet} {bright_cyan}{username}{reset}"),
            }
        }
        let count = headers.x_total_count.unwrap_or_default() as u64;
        println!("Page {} of {}", page.unwrap_or(1), count.div_ceil(20));
    }
    Ok(())
}

async fn add_user_to_team(
    api: &Forgejo,
    org: String,
    team: String,
    user: String,
) -> eyre::Result<()> {
    let id = find_team_by_name(api, &org, &team)
        .await?
        .id
        .ok_or_eyre("team does not have id")?;
    api.org_add_team_member(id as u64, &user).await?;
    let SpecialRender {
        bold,
        reset,
        bright_blue,
        bright_cyan,
        ..
    } = crate::special_render();
    println!("Added {bright_cyan}{bold}{user}{reset} to team {bright_blue}{bold}{team}{reset}");
    Ok(())
}

async fn remove_user_from_team(
    api: &Forgejo,
    org: String,
    team: String,
    user: String,
) -> eyre::Result<()> {
    let id = find_team_by_name(api, &org, &team)
        .await?
        .id
        .ok_or_eyre("team does not have id")?;
    api.org_remove_team_member(id as u64, &user).await?;
    let SpecialRender {
        bold,
        reset,
        bright_blue,
        bright_cyan,
        ..
    } = crate::special_render();
    println!("Removed {bright_cyan}{bold}{user}{reset} from team {bright_blue}{bold}{team}{reset}");
    Ok(())
}
