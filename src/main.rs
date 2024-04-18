use clap::{Parser, Subcommand};
use eyre::{eyre, Context, OptionExt};
use tokio::io::AsyncWriteExt;

mod keys;
use keys::*;

mod auth;
mod issues;
mod release;
mod repo;

#[derive(Parser, Debug)]
pub struct App {
    #[clap(long, short = 'H')]
    host: Option<String>,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    #[clap(subcommand)]
    Repo(repo::RepoCommand),
    Issue(issues::IssueCommand),
    #[command(name = "whoami")]
    WhoAmI {
        #[clap(long, short)]
        remote: Option<String>,
    },
    #[clap(subcommand)]
    Auth(auth::AuthCommand),
    Release(release::ReleaseCommand),
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args = App::parse();
    let mut keys = KeyInfo::load().await?;

    let host_name = args.host.as_deref();
    // let remote = repo::RepoInfo::get_current(host_name, remote_name)?;
    match args.command {
        Command::Repo(subcommand) => subcommand.run(&keys, host_name).await?,
        Command::Issue(subcommand) => subcommand.run(&keys, host_name).await?,
        Command::WhoAmI { remote } => {
            let url = repo::RepoInfo::get_current(host_name, None, remote.as_deref())
                .wrap_err("could not find host, try specifying with --host")?
                .host_url()
                .clone();
            let name = keys.get_login(&url)?.username();
            let host = url
                .host_str()
                .ok_or_eyre("instance url does not have host")?;
            if url.path() == "/" || url.path().is_empty() {
                println!("currently signed in to {name}@{host}");
            } else {
                println!("currently signed in to {name}@{host}{}", url.path());
            }
        }
        Command::Auth(subcommand) => subcommand.run(&mut keys).await?,
        Command::Release(subcommand) => subcommand.run(&mut keys, host_name).await?,
    }

    keys.save().await?;
    Ok(())
}

async fn readline(msg: &str) -> eyre::Result<String> {
    print!("{msg}");
    tokio::io::stdout().flush().await?;
    tokio::task::spawn_blocking(|| {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        Ok(input)
    })
    .await?
}

async fn editor(contents: &mut String, ext: Option<&str>) -> eyre::Result<()> {
    let editor = std::path::PathBuf::from(
        std::env::var_os("EDITOR").ok_or_else(|| eyre!("unable to locate editor"))?,
    );

    let (mut file, path) = tempfile(ext).await?;
    file.write_all(contents.as_bytes()).await?;
    drop(file);

    // Closure acting as a try/catch block so that the temp file is deleted even
    // on errors
    let res = (|| async {
        eprint!("waiting on editor\r");
        let flags = get_editor_flags(&editor);
        let status = tokio::process::Command::new(editor)
            .args(flags)
            .arg(&path)
            .status()
            .await?;
        if !status.success() {
            eyre::bail!("editor exited unsuccessfully");
        }

        *contents = tokio::fs::read_to_string(&path).await?;
        eprint!("                 \r");

        Ok(())
    })()
    .await;

    tokio::fs::remove_file(path).await?;
    res?;
    Ok(())
}

fn get_editor_flags(editor_path: &std::path::Path) -> &'static [&'static str] {
    let editor_name = match editor_path.file_stem().and_then(|s| s.to_str()) {
        Some(name) => name,
        None => return &[],
    };
    if editor_name == "code" {
        return &["--wait"];
    }
    &[]
}

async fn tempfile(ext: Option<&str>) -> tokio::io::Result<(tokio::fs::File, std::path::PathBuf)> {
    let filename = uuid::Uuid::new_v4();
    let mut path = std::env::temp_dir().join(filename.to_string());
    if let Some(ext) = ext {
        path.set_extension(ext);
    }
    let file = tokio::fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&path)
        .await?;
    Ok((file, path))
}
