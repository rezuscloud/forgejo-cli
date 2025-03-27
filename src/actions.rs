use std::fmt::Display;

use clap::{Args, Subcommand};
use eyre::{bail, OptionExt};
use forgejo_api::{
    structs::{
        ActionVariable, CreateOrUpdateSecretOption, CreateVariableOption,
        GetRepoVariablesListQuery, RepoListActionsSecretsQuery, UpdateVariableOption,
    },
    Forgejo, ForgejoError,
};
use hyper::StatusCode;
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

    Secrets {
        #[clap(subcommand)]
        command: ActionsSecretsSubcommmand,
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

    /// Create a new variable
    Create {
        /// The name of the new variable
        name: String,

        /// The data to save into the variable. Omit to invoke editor.
        data: Option<String>,

        /// Override existing variables
        #[clap(long, short)]
        force: bool,
    },

    Delete {
        /// The variable to delete
        name: String,
    },
}

#[derive(Subcommand, Clone, Debug)]
pub enum ActionsSecretsSubcommmand {
    /// List secrets
    List,

    /// Create a new secret
    Create {
        /// The name of the new secret
        name: String,

        /// The data to save into the secret. Omit to invoke editor.
        data: Option<String>,
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
            ActionsSubcommand::Variables {
                command: ActionsVariablesSubcommmand::Create { name, data, force },
            } => create_variable(repo, &api, name, data, force).await?,
            ActionsSubcommand::Variables {
                command: ActionsVariablesSubcommmand::Delete { name },
            } => delete_variable(repo, &api, name).await?,

            ActionsSubcommand::Secrets {
                command: ActionsSecretsSubcommmand::List,
            } => list_secrets(repo, &api).await?,
            ActionsSubcommand::Secrets {
                command: ActionsSecretsSubcommmand::Create { name, data },
            } => create_secret(repo, &api, name, data).await?,
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
            "#{bold}{}{reset} ({bright_blue}{}{reset}) {} {} {bright_green}{}{reset} ({}): {yellow}{}{reset}",
            task.run_number.unwrap_or(0),
            sha,
            task_sym,
            task.name.unwrap_or_default(),
            time,
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
        println!("{}", DisplayActionVariable::new(var, verbose)?);
    }

    Ok(())
}

async fn create_variable(
    repo: &RepoName,
    api: &Forgejo,
    name: String,
    data: Option<String>,
    force: bool,
) -> eyre::Result<()> {
    let mut data = if let Some(data) = data {
        data
    } else {
        let mut data = String::new();
        crate::editor(&mut data, Some("variable_content.txt")).await?;
        data
    };

    match api
        .create_repo_variable(
            repo.owner(),
            repo.name(),
            &name,
            CreateVariableOption {
                // If we don't have force enabled, we will not need the data again to (potentially)
                // make another request. To avoid a clone in this case, we take the string here,
                // replacing it with an empty one.
                value: if force {
                    data.clone()
                } else {
                    std::mem::take(&mut data)
                },
            },
        )
        .await
    {
        Err(ForgejoError::ApiError(StatusCode::CONFLICT, _)) => {
            if !force {
                bail!("variable already exists, pass --force to replace it.");
            }

            eprintln!("variable already exists, updating.");
            api.update_repo_variable(
                repo.owner(),
                repo.name(),
                &name,
                UpdateVariableOption {
                    name: None,
                    value: data,
                },
            )
            .await?;
        }
        Err(e) => return Err(e.into()),
        Ok(()) => {}
    }

    Ok(())
}

async fn delete_variable(repo: &RepoName, api: &Forgejo, name: String) -> eyre::Result<()> {
    let var = api
        .delete_repo_variable(repo.owner(), repo.name(), &name)
        .await?;

    if let Some(var) = var {
        println!("Deleted: {}", DisplayActionVariable::new(var, false)?);
    }

    Ok(())
}

async fn list_secrets(repo: &RepoName, api: &Forgejo) -> eyre::Result<()> {
    let per_page = 64;
    let mut secrets = vec![];

    for page in 1.. {
        let (_headers, page_secrets) = api
            .repo_list_actions_secrets(
                repo.owner(),
                repo.name(),
                RepoListActionsSecretsQuery {
                    page: Some(page),
                    limit: Some(per_page),
                },
            )
            .await?;

        let done = page_secrets.len() < per_page as usize;
        secrets.extend(page_secrets.into_iter());
        if done {
            break;
        }
    }

    for secret in secrets {
        println!(
            "({}) {}",
            crate::DisplayOptional(secret.created_at, "?"),
            crate::DisplayOptional(secret.name, "?")
        );
    }

    Ok(())
}

async fn create_secret(
    repo: &RepoName,
    api: &Forgejo,
    name: String,
    data: Option<String>,
) -> eyre::Result<()> {
    let data = if let Some(data) = data {
        data
    } else {
        let mut data = String::new();
        crate::editor(&mut data, Some("secret_content.txt")).await?;
        data
    };

    api.update_repo_secret(
        repo.owner(),
        repo.name(),
        &name,
        CreateOrUpdateSecretOption { data },
    )
    .await?;

    Ok(())
}

struct DisplayActionVariable {
    name: String,
    data: String,
    owner_id: Option<i64>,
    repo_id: Option<i64>,
    verbose: bool,
}

impl DisplayActionVariable {
    fn new(value: ActionVariable, verbose: bool) -> eyre::Result<Self> {
        Ok(Self {
            name: value
                .name
                .ok_or_eyre("Server returned ActionVariable without name?!")?,
            // The API usually (always?) returns Some("") here. The page on variables also notes
            // that their value cannot be read by other means than being passed to a CI job.
            data: value.data.unwrap_or_default(),
            owner_id: value.owner_id,
            repo_id: value.repo_id,
            verbose,
        })
    }
}

impl Display for DisplayActionVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.verbose {
            write!(
                f,
                "({}, {}) ",
                crate::DisplayOptional(self.owner_id, "?"),
                crate::DisplayOptional(self.repo_id, "?"),
            )?;
        }

        write!(f, "{}", self.name)?;

        if !self.data.is_empty() {
            write!(f, " = {}", self.data)?;
        }

        Ok(())
    }
}
