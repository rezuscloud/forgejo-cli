use std::io::IsTerminal;

use clap::{Parser, Subcommand};
use eyre::{eyre, Context, OptionExt};
use tokio::io::AsyncWriteExt;

mod keys;
use keys::*;

mod auth;
mod issues;
mod prs;
mod release;
mod repo;

#[derive(Parser, Debug)]
pub struct App {
    #[clap(long, short = 'H')]
    host: Option<String>,
    #[clap(long)]
    style: Option<Style>,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    #[clap(subcommand)]
    Repo(repo::RepoCommand),
    Issue(issues::IssueCommand),
    Pr(prs::PrCommand),
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

    let _ = SPECIAL_RENDER.set(SpecialRender::new(args.style.unwrap_or_default()));

    let mut keys = KeyInfo::load().await?;

    let host_name = args.host.as_deref();
    // let remote = repo::RepoInfo::get_current(host_name, remote_name)?;
    match args.command {
        Command::Repo(subcommand) => subcommand.run(&mut keys, host_name).await?,
        Command::Issue(subcommand) => subcommand.run(&mut keys, host_name).await?,
        Command::Pr(subcommand) => subcommand.run(&mut keys, host_name).await?,
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
        Command::Auth(subcommand) => subcommand.run(&mut keys, host_name).await?,
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

use std::sync::OnceLock;
static SPECIAL_RENDER: OnceLock<SpecialRender> = OnceLock::new();

fn special_render() -> &'static SpecialRender {
    SPECIAL_RENDER
        .get()
        .expect("attempted to get special characters before that was initialized")
}

#[derive(clap::ValueEnum, Clone, Copy, Debug, Default)]
enum Style {
    /// Use special characters, and colors.
    #[default]
    Fancy,
    /// No special characters and no colors. Always used in non-terminal contexts (i.e. pipes)
    Minimal,
}

struct SpecialRender {
    colors: bool,

    dash: char,
    bullet: char,
    body_prefix: char,

    red: &'static str,
    bright_red: &'static str,
    green: &'static str,
    bright_green: &'static str,
    blue: &'static str,
    bright_blue: &'static str,
    cyan: &'static str,
    bright_cyan: &'static str,
    yellow: &'static str,
    bright_yellow: &'static str,
    magenta: &'static str,
    bright_magenta: &'static str,
    black: &'static str,
    dark_grey: &'static str,
    light_grey: &'static str,
    white: &'static str,
    reset: &'static str,

    hide_cursor: &'static str,
    show_cursor: &'static str,
    clear_line: &'static str,
}

impl SpecialRender {
    fn new(display: Style) -> Self {
        let is_tty = std::io::stdout().is_terminal();
        match display {
            _ if !is_tty => Self::minimal(),
            Style::Fancy => Self::fancy(),
            Style::Minimal => Self::minimal(),
        }
    }

    fn fancy() -> Self {
        Self {
            colors: true,

            dash: '—',
            bullet: '•',
            body_prefix: '▌',

            red: "\x1b[31m",
            bright_red: "\x1b[91m",
            green: "\x1b[32m",
            bright_green: "\x1b[92m",
            blue: "\x1b[34m",
            bright_blue: "\x1b[94m",
            cyan: "\x1b[36m",
            bright_cyan: "\x1b[96m",
            yellow: "\x1b[33m",
            bright_yellow: "\x1b[93m",
            magenta: "\x1b[35m",
            bright_magenta: "\x1b[95m",
            black: "\x1b[30m",
            dark_grey: "\x1b[90m",
            light_grey: "\x1b[37m",
            white: "\x1b[97m",
            reset: "\x1b[0m",

            hide_cursor: "\x1b[?25l",
            show_cursor: "\x1b[?25h",
            clear_line: "\x1b[2K",
        }
    }

    fn minimal() -> Self {
        Self {
            colors: false,

            dash: '-',
            bullet: '-',
            body_prefix: '>',

            red: "",
            bright_red: "",
            green: "",
            bright_green: "",
            blue: "",
            bright_blue: "",
            cyan: "",
            bright_cyan: "",
            yellow: "",
            bright_yellow: "",
            magenta: "",
            bright_magenta: "",
            black: "",
            dark_grey: "",
            light_grey: "",
            white: "",
            reset: "",

            hide_cursor: "",
            show_cursor: "",
            clear_line: "",
        }
    }
}
