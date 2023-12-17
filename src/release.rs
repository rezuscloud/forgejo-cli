use clap::Subcommand;
use eyre::{bail, eyre};
use forgejo_api::Forgejo;
use tokio::io::AsyncWriteExt;

use crate::{keys::KeyInfo, repo::RepoInfo};

#[derive(Subcommand, Clone, Debug)]
pub enum ReleaseCommand {
    Create {
        name: String,
        #[clap(long, short = 'T')]
        /// Create a new cooresponding tag for this release. Defaults to release's name.
        create_tag: Option<Option<String>>,
        #[clap(long, short = 't')]
        /// Pre-existing tag to use
        ///
        /// If you need to create a new tag for this release, use `--create-tag`
        tag: Option<String>,
        #[clap(
            long,
            short,
            help = "Include a file as an attachment",
            long_help = "Include a file as an attachment
        
`--attach=<FILE>` will set the attachment's name to the file name
`--attach=<FILE>:<ASSET>` will use the provided name for the attachment"
        )]
        attach: Vec<String>,
        #[clap(long, short)]
        /// Text of the release body.
        ///
        /// Using this flag without an argument will open your editor.
        body: Option<Option<String>>,
        #[clap(long, short = 'B')]
        branch: Option<String>,
        #[clap(long, short)]
        draft: bool,
        #[clap(long, short)]
        prerelease: bool,
    },
    Edit {
        name: String,
        #[clap(long, short = 'n')]
        rename: Option<String>,
        #[clap(long, short = 't')]
        /// Corresponding tag for this release.
        tag: Option<String>,
        #[clap(long, short)]
        /// Text of the release body.
        ///
        /// Using this flag without an argument will open your editor.
        body: Option<Option<String>>,
        #[clap(long, short)]
        draft: Option<bool>,
        #[clap(long, short)]
        prerelease: Option<bool>,
    },
    Delete {
        name: String,
        #[clap(long, short = 't')]
        by_tag: bool,
    },
    List {
        #[clap(long, short = 'p')]
        include_prerelease: bool,
        #[clap(long, short = 'd')]
        include_draft: bool,
    },
    View {
        name: String,
        #[clap(long, short = 't')]
        by_tag: bool,
    },
    Browse {
        name: Option<String>,
    },
    #[clap(subcommand)]
    Asset(AssetCommand),
}

#[derive(Subcommand, Clone, Debug)]
pub enum AssetCommand {
    Create {
        release: String,
        path: std::path::PathBuf,
        name: Option<String>,
    },
    Delete {
        release: String,
        asset: String,
    },
    Download {
        release: String,
        asset: String,
        #[clap(long, short)]
        output: Option<std::path::PathBuf>,
    },
}

impl ReleaseCommand {
    pub async fn run(self, keys: &KeyInfo, remote_name: Option<&str>) -> eyre::Result<()> {
        let repo = RepoInfo::get_current(remote_name)?;
        let api = keys.get_api(&repo.host_url())?;
        match self {
            Self::Create {
                name,
                create_tag,
                tag,
                attach,
                body,
                branch,
                draft,
                prerelease,
            } => {
                create_release(
                    &repo, &api, name, create_tag, tag, attach, body, branch, draft, prerelease,
                )
                .await?
            }
            Self::Edit {
                name,
                rename,
                tag,
                body,
                draft,
                prerelease,
            } => edit_release(&repo, &api, name, rename, tag, body, draft, prerelease).await?,
            Self::Delete { name, by_tag } => delete_release(&repo, &api, name, by_tag).await?,
            Self::List {
                include_prerelease,
                include_draft,
            } => list_releases(&repo, &api, include_prerelease, include_draft).await?,
            Self::View { name, by_tag } => view_release(&repo, &api, name, by_tag).await?,
            Self::Browse { name } => browse_release(&repo, &api, name).await?,
            Self::Asset(subcommand) => match subcommand {
                AssetCommand::Create {
                    release,
                    path,
                    name,
                } => create_asset(&repo, &api, release, path, name).await?,
                AssetCommand::Delete { release, asset } => {
                    delete_asset(&repo, &api, release, asset ).await?
                }
                AssetCommand::Download {
                    release,
                    asset,
                    output,
                } => download_asset(&repo, &api, release, asset, output).await?,
            },
        }
        Ok(())
    }
}

async fn create_release(
    repo: &RepoInfo,
    api: &Forgejo,
    name: String,
    create_tag: Option<Option<String>>,
    tag: Option<String>,
    attachments: Vec<String>,
    body: Option<Option<String>>,
    branch: Option<String>,
    draft: bool,
    prerelease: bool,
) -> eyre::Result<()> {
    let tag_name = match (tag, create_tag) {
        (None, None) => bail!("must select tag with `--tag` or `--create-tag`"),
        (Some(tag), None) => tag,
        (None, Some(tag)) => {
            let tag = tag.unwrap_or_else(|| name.clone());
            let opt = forgejo_api::CreateTagOption {
                message: None,
                tag_name: tag.clone(),
                target: branch,
            };
            api.create_tag(repo.owner(), repo.name(), opt).await?;
            tag
        }
        (Some(_), Some(_)) => {
            bail!("`--tag` and `--create-tag` are mutually exclusive; please pick just one")
        }
    };

    let body = match body {
        Some(Some(body)) => body,
        Some(None) => {
            let mut s = String::new();
            crate::editor(&mut s, Some("md")).await?;
            s
        }
        None => String::new(),
    };

    let release_opt = forgejo_api::CreateReleaseOption {
        body,
        draft,
        name,
        prerelease,
        tag_name,
        target_commitish: None,
    };
    let release = api
        .create_release(repo.owner(), repo.name(), release_opt)
        .await?;

    for attachment in attachments {
        let (file, asset) = match attachment.split_once(':') {
            Some((file, asset)) => (std::path::Path::new(file), asset),
            None => {
                let file = std::path::Path::new(&attachment);
                let asset = file
                    .file_name()
                    .ok_or_else(|| eyre!("{attachment} does not have a file name"))?
                    .to_str()
                    .unwrap();
                (file, asset)
            }
        };
        api.create_release_attachment(
            repo.owner(),
            repo.name(),
            release.id,
            asset,
            tokio::fs::read(file).await?,
        )
        .await?;
    }

    Ok(())
}

async fn edit_release(
    repo: &RepoInfo,
    api: &Forgejo,
    name: String,
    rename: Option<String>,
    tag: Option<String>,
    body: Option<Option<String>>,
    draft: Option<bool>,
    prerelease: Option<bool>,
) -> eyre::Result<()> {
    let release = find_release(repo, api, &name).await?;
    let body = match body {
        Some(Some(body)) => Some(body),
        Some(None) => {
            let mut s = release.body.clone();
            crate::editor(&mut s, Some("md")).await?;
            Some(s)
        }
        None => None,
    };
    let release_edit = forgejo_api::EditReleaseOption {
        name: rename,
        tag_name: tag,
        body,
        draft,
        prerelease,
        target_commitish: None,
    };
    api.edit_release(repo.owner(), repo.name(), release.id, release_edit)
        .await?;
    Ok(())
}

async fn list_releases(
    repo: &RepoInfo,
    api: &Forgejo,
    prerelease: bool,
    draft: bool,
) -> eyre::Result<()> {
    let query = forgejo_api::ReleaseQuery {
        prerelease: Some(prerelease),
        draft: Some(draft),
        ..Default::default()
    };
    let releases = api.get_releases(repo.owner(), repo.name(), query).await?;
    for release in releases {
        print!("{}", release.name);
        match (release.draft, release.prerelease) {
            (false, false) => (),
            (true, false) => print!(" (draft)"),
            (false, true) => print!(" (prerelease)"),
            (true, true) => print!(" (draft, prerelease)"),
        }
        println!();
    }
    Ok(())
}

async fn view_release(
    repo: &RepoInfo,
    api: &Forgejo,
    name: String,
    by_tag: bool,
) -> eyre::Result<()> {
    let release = if by_tag {
        api.get_release_by_tag(repo.owner(), repo.name(), &name)
            .await?
            .ok_or_else(|| eyre!("release not found"))?
    } else {
        find_release(repo, api, &name).await?
    };
    println!("{}", release.name);
    print!("By {} on ", release.author.login);
    release.created_at.format_into(
        &mut std::io::stdout(),
        &time::format_description::well_known::Rfc2822,
    )?;
    println!();
    if !release.body.is_empty() {
        println!();
        for line in release.body.lines() {
            println!("> {line}");
        }
        println!();
    }
    if !release.assets.is_empty() {
        println!("{} assets", release.assets.len() + 2);
        for asset in release.assets {
            println!("- {}", asset.name);
        }
        println!("- source.zip");
        println!("- source.tar.gz");
    }
    Ok(())
}

async fn browse_release(repo: &RepoInfo, api: &Forgejo, name: Option<String>) -> eyre::Result<()> {
    match name {
        Some(name) => {
            let release = find_release(repo, api, &name).await?;
            open::that(release.html_url.as_str())?;
        }
        None => {
            let mut url = repo.url().clone();
            url.path_segments_mut().unwrap().push("releases");
            open::that(url.as_str())?;
        }
    }
    Ok(())
}

async fn create_asset(
    repo: &RepoInfo,
    api: &Forgejo,
    release: String,
    file: std::path::PathBuf,
    asset: Option<String>,
) -> eyre::Result<()> {
    let (file, asset) = match asset {
        Some(ref asset) => (&*file, &**asset),
        None => {
            let asset = file
                .file_name()
                .ok_or_else(|| eyre!("{} does not have a file name", file.display()))?
                .to_str()
                .unwrap();
            (&*file, asset)
        }
    };
    let id = find_release(repo, api, &release).await?.id;
    api.create_release_attachment(
        repo.owner(),
        repo.name(),
        id,
        asset,
        tokio::fs::read(file).await?,
    )
    .await?;

    Ok(())
}

async fn delete_asset(
    repo: &RepoInfo,
    api: &Forgejo,
    release: String,
    asset: String,
) -> eyre::Result<()> {
    let release = find_release(repo, api, &release).await?;
    let asset = release
        .assets
        .iter()
        .find(|a| a.name == asset)
        .ok_or_else(|| eyre!("asset not found"))?;
    api.delete_release_attachment(repo.owner(), repo.name(), release.id, asset.id)
        .await?;
    Ok(())
}

async fn download_asset(
    repo: &RepoInfo,
    api: &Forgejo,
    release: String,
    asset: String,
    output: Option<std::path::PathBuf>,
) -> eyre::Result<()> {
    let release = find_release(repo, api, &release).await?;
    let file = match &*asset {
        "source.zip" => api.download_release_zip(repo.owner(), repo.name(), release.id).await?,
        "source.tar.gz" => api.download_release_tarball(repo.owner(), repo.name(), release.id).await?,
        name => {
            let asset = release
                .assets
                .iter()
                .find(|a| a.name == name)
                .ok_or_else(|| eyre!("asset not found"))?;
            api.download_release_attachment(repo.owner(), repo.name(), release.id, asset.id).await?
        }
    };
    let file = file.ok_or_else(|| eyre!("asset not found"))?;
    let output = output
        .as_deref()
        .unwrap_or_else(|| std::path::Path::new(&asset));
    tokio::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(output)
        .await?
        .write_all(file.as_ref())
        .await?;

    Ok(())
}

async fn find_release(
    repo: &RepoInfo,
    api: &Forgejo,
    name: &str,
) -> eyre::Result<forgejo_api::Release> {
    let mut releases = api
        .get_releases(
            repo.owner(),
            repo.name(),
            forgejo_api::ReleaseQuery::default(),
        )
        .await?;
    let idx = releases
        .iter()
        .position(|r| r.name == name)
        .ok_or_else(|| eyre!("release not found"))?;
    Ok(releases.swap_remove(idx))
}

async fn delete_release(
    repo: &RepoInfo,
    api: &Forgejo,
    name: String,
    by_tag: bool,
) -> eyre::Result<()> {
    if by_tag {
        api.delete_release_by_tag(repo.owner(), repo.name(), &name)
            .await?;
    } else {
        let id = find_release(repo, api, &name).await?.id;
        api.delete_release(repo.owner(), repo.name(), id).await?;
    }
    Ok(())
}
