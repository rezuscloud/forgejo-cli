use clap::{Args, Subcommand};
use eyre::OptionExt;
use forgejo_api::{structs::GetRepoVariablesListQuery, Forgejo};
use time::Duration;

use crate::{
    repo::{RepoArg, RepoInfo, RepoName},
    SpecialRender,
};

#[derive(Args, Clone, Debug)]
pub struct ActionsCommand {
    /// The local git remote that points to the repo to operate on.
    #[clap(long, short = 'R')]
    remote: Option<String>,

    #[clap(long, short = 'r')]
    repo: Option<RepoArg>,

    #[clap(subcommand)]
    command: ActionsSubcommand,
}

#[derive(Subcommand, Clone, Debug)]
pub enum ActionsSubcommand {
    /// List the tasks on a repo
    Tasks,

    /// List and manage variables
    Variables {
        #[clap(subcommand)]
        command: ActionsVariablesSubcommmand,
    },
}

#[derive(Subcommand, Clone, Debug)]
pub enum ActionsVariablesSubcommmand {
    /// List variables
    List {
        /// Also print owner_id and repo_id
        #[clap(long, short)]
        verbose: bool,
    },
}

impl ActionsCommand {
    pub async fn run(self, keys: &mut crate::KeyInfo, host_name: Option<&str>) -> eyre::Result<()> {
        let repo =
            RepoInfo::get_current(host_name, self.repo.as_ref(), self.remote.as_deref(), &keys)?;

        let api = keys.get_api(repo.host_url()).await?;
        let repo = repo
            .name()
            .ok_or_eyre("can't figure what repo to access, try specifying with `--repo`")?;
        match self.command {
            ActionsSubcommand::Tasks => view_tasks(repo, &api).await?,
            ActionsSubcommand::Variables {
                command: ActionsVariablesSubcommmand::List { verbose },
            } => list_variables(repo, &api, verbose).await?,
        }

        Ok(())
    }
}

async fn view_tasks(repo: &RepoName, api: &Forgejo) -> eyre::Result<()> {
    // We don't iterate this to collect all tasks (not just the ones on the first page) like the
    // issue search subcommand will do, because it's unlikely someone wants to see *all* tasks.
    let res = api
        .list_action_tasks(
            repo.owner(),
            repo.name(),
            forgejo_api::structs::ListActionTasksQuery {
                page: None,
                limit: None,
            },
        )
        .await?;

    if res.total_count == Some(1) {
        println!("1 task");
    } else {
        println!("{} tasks", res.total_count.unwrap_or(0));
    }

    let SpecialRender {
        fancy,
        reset,

        bold,
        bright_green,
        light_grey,
        bright_red,
        yellow,
        bright_blue,
        ..
    } = *crate::special_render();

    for task in res.workflow_runs.unwrap_or_default() {
        let task_sym = match task.status.as_deref() {
            // Don't use symbols when we're not in fancy mode.
            x if !fancy => x.unwrap_or("?"),

            // See: https://codeberg.org/forgejo/forgejo/src/branch/forgejo/models/actions/status.go#L26
            Some("success") => &format!("{bright_green}✓{reset}"),
            Some("cancelled") => &format!("{light_grey}!{reset}"),
            Some("failure") => &format!("{bright_red}×{reset}"),
            Some("waiting") => &format!("{light_grey}{reset}"),
            Some("running") => &format!("{yellow}●{reset}"),
            Some("skipped") => &format!("{light_grey}{reset}"),
            Some("blocked") => &format!("{bright_red}{reset}"),
            Some(x) => x,
            None => "?",
        };

        let sha = task.head_sha.unwrap_or_default();
        let sha = if sha.len() > 10 { &sha[0..10] } else { &sha };

        let time = if let (Some(end), Some(start)) = (task.updated_at, task.run_started_at) {
            end - start
        } else {
            Duration::default()
        };

        println!(
            "#{bold}{}{reset} ({bright_blue}{}{reset}) {} {bright_green}{}{reset} {} ({}): {yellow}{}{reset}",
            task.run_number.unwrap_or(0),
            sha,
            task_sym,
            time,
            task.name.unwrap_or_default(),
            task.event.unwrap_or_default(),
            task.display_title.unwrap_or_default(),
        );
    }

    Ok(())
}

async fn list_variables(repo: &RepoName, api: &Forgejo, verbose: bool) -> eyre::Result<()> {
    let per_page = 64;
    let mut variables = vec![];

    for page in 1.. {
        let (_headers, vars) = api
            .get_repo_variables_list(
                repo.owner(),
                repo.name(),
                GetRepoVariablesListQuery {
                    page: Some(page),
                    limit: Some(per_page),
                },
            )
            .await?;

        let done = vars.len() < per_page as usize;
        variables.extend(vars.into_iter());
        if done {
            break;
        }
    }

    for var in variables {
        if let Some(name) = var.name {
            let prefix = if verbose {
                format!(
                    "({}, {}) ",
                    crate::DisplayOptional(var.owner_id, "?"),
                    crate::DisplayOptional(var.repo_id, "?"),
                )
            } else {
                String::new()
            };

            // The API usually (always?) returns Some("") here. The page on variables also notes
            // that their value cannot be read by other means than being passed to a CI job.
            let data = var.data.unwrap_or_default();
            if data.is_empty() {
                println!("{prefix}{name}");
            } else {
                println!("{prefix}{name} = {data}");
            }
        }
    }

    Ok(())
}

