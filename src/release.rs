use clap::{Args, Subcommand};
use eyre::{bail, eyre, OptionExt};
use forgejo_api::{
    structs::{RepoCreateReleaseAttachmentQuery, RepoListReleasesQuery},
    Forgejo,
};
use tokio::io::AsyncWriteExt;

use crate::{
    keys::KeyInfo,
    repo::{RepoInfo, RepoName},
    SpecialRender,
};

#[derive(Args, Clone, Debug)]
pub struct ReleaseCommand {
    #[clap(long, short = 'R')]
    remote: Option<String>,
    #[clap(long, short)]
    repo: Option<String>,
    #[clap(subcommand)]
    command: ReleaseSubcommand,
}

#[derive(Subcommand, Clone, Debug)]
pub enum ReleaseSubcommand {
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
    pub async fn run(self, keys: &mut KeyInfo, remote_name: Option<&str>) -> eyre::Result<()> {
        let repo =
            RepoInfo::get_current(remote_name, self.repo.as_deref(), self.remote.as_deref())?;
        let api = keys.get_api(&repo.host_url()).await?;
        let repo = repo
            .name()
            .ok_or_eyre("couldn't get repo name, try specifying with --repo")?;
        match self.command {
            ReleaseSubcommand::Create {
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
            ReleaseSubcommand::Edit {
                name,
                rename,
                tag,
                body,
                draft,
                prerelease,
            } => edit_release(&repo, &api, name, rename, tag, body, draft, prerelease).await?,
            ReleaseSubcommand::Delete { name, by_tag } => {
                delete_release(&repo, &api, name, by_tag).await?
            }
            ReleaseSubcommand::List {
                include_prerelease,
                include_draft,
            } => list_releases(&repo, &api, include_prerelease, include_draft).await?,
            ReleaseSubcommand::View { name, by_tag } => {
                view_release(&repo, &api, name, by_tag).await?
            }
            ReleaseSubcommand::Browse { name } => browse_release(&repo, &api, name).await?,
            ReleaseSubcommand::Asset(subcommand) => match subcommand {
                AssetCommand::Create {
                    release,
                    path,
                    name,
                } => create_asset(&repo, &api, release, path, name).await?,
                AssetCommand::Delete { release, asset } => {
                    delete_asset(&repo, &api, release, asset).await?
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
    repo: &RepoName,
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
            let opt = forgejo_api::structs::CreateTagOption {
                message: None,
                tag_name: tag.clone(),
                target: branch,
            };
            api.repo_create_tag(repo.owner(), repo.name(), opt).await?;
            tag
        }
        (Some(_), Some(_)) => {
            bail!("`--tag` and `--create-tag` are mutually exclusive; please pick just one")
        }
    };

    let body = match body {
        Some(Some(body)) => Some(body),
        Some(None) => {
            let mut s = String::new();
            crate::editor(&mut s, Some("md")).await?;
            Some(s)
        }
        None => None,
    };

    let release_opt = forgejo_api::structs::CreateReleaseOption {
        body,
        draft: Some(draft),
        name: Some(name),
        prerelease: Some(prerelease),
        tag_name,
        target_commitish: None,
    };
    let release = api
        .repo_create_release(repo.owner(), repo.name(), release_opt)
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
        let query = RepoCreateReleaseAttachmentQuery {
            name: Some(asset.into()),
        };
        let id = release
            .id
            .ok_or_else(|| eyre::eyre!("release does not have id"))?;
        api.repo_create_release_attachment(
            repo.owner(),
            repo.name(),
            id,
            tokio::fs::read(file).await?,
            query,
        )
        .await?;
    }

    Ok(())
}

async fn edit_release(
    repo: &RepoName,
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
            let mut s = release
                .body
                .clone()
                .ok_or_else(|| eyre::eyre!("release does not have body"))?;
            crate::editor(&mut s, Some("md")).await?;
            Some(s)
        }
        None => None,
    };
    let release_edit = forgejo_api::structs::EditReleaseOption {
        name: rename,
        tag_name: tag,
        body,
        draft,
        prerelease,
        target_commitish: None,
    };
    let id = release
        .id
        .ok_or_else(|| eyre::eyre!("release does not have id"))?;
    api.repo_edit_release(repo.owner(), repo.name(), id, release_edit)
        .await?;
    Ok(())
}

async fn list_releases(
    repo: &RepoName,
    api: &Forgejo,
    prerelease: bool,
    draft: bool,
) -> eyre::Result<()> {
    let query = forgejo_api::structs::RepoListReleasesQuery {
        pre_release: Some(prerelease),
        draft: Some(draft),
        page: None,
        limit: None,
    };
    let releases = api
        .repo_list_releases(repo.owner(), repo.name(), query)
        .await?;
    for release in releases {
        let name = release
            .name
            .as_ref()
            .ok_or_else(|| eyre::eyre!("release does not have name"))?;
        let draft = release
            .draft
            .as_ref()
            .ok_or_else(|| eyre::eyre!("release does not have draft"))?;
        let prerelease = release
            .prerelease
            .as_ref()
            .ok_or_else(|| eyre::eyre!("release does not have prerelease"))?;
        print!("{}", name);
        match (draft, prerelease) {
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
    repo: &RepoName,
    api: &Forgejo,
    name: String,
    by_tag: bool,
) -> eyre::Result<()> {
    let release = if by_tag {
        api.repo_get_release_by_tag(repo.owner(), repo.name(), &name)
            .await?
    } else {
        find_release(repo, api, &name).await?
    };
    let name = release
        .name
        .as_ref()
        .ok_or_else(|| eyre::eyre!("release does not have name"))?;
    let author = release
        .author
        .as_ref()
        .ok_or_else(|| eyre::eyre!("release does not have author"))?;
    let login = author
        .login
        .as_ref()
        .ok_or_else(|| eyre::eyre!("autho does not have login"))?;
    let created_at = release
        .created_at
        .ok_or_else(|| eyre::eyre!("release does not have created_at"))?;
    println!("{}", name);
    print!("By {} on ", login);
    created_at.format_into(
        &mut std::io::stdout(),
        &time::format_description::well_known::Rfc2822,
    )?;
    println!();
    let SpecialRender { bullet, .. } = crate::special_render();
    let body = release
        .body
        .as_ref()
        .ok_or_else(|| eyre::eyre!("release does not have body"))?;
    if !body.is_empty() {
        println!();
        println!("{}", crate::markdown(&body));
        println!();
    }
    let assets = release
        .assets
        .as_ref()
        .ok_or_else(|| eyre::eyre!("release does not have assets"))?;
    if !assets.is_empty() {
        println!("{} assets", assets.len() + 2);
        for asset in assets {
            let name = asset
                .name
                .as_ref()
                .ok_or_else(|| eyre::eyre!("asset does not have name"))?;
            println!("{bullet} {}", name);
        }
        println!("{bullet} source.zip");
        println!("{bullet} source.tar.gz");
    }
    Ok(())
}

async fn browse_release(repo: &RepoName, api: &Forgejo, name: Option<String>) -> eyre::Result<()> {
    match name {
        Some(name) => {
            let release = find_release(repo, api, &name).await?;
            let html_url = release
                .html_url
                .as_ref()
                .ok_or_else(|| eyre::eyre!("release does not have html_url"))?;
            open::that(html_url.as_str())?;
        }
        None => {
            let repo_data = api.repo_get(repo.owner(), repo.name()).await?;
            let mut html_url = repo_data
                .html_url
                .clone()
                .ok_or_else(|| eyre::eyre!("repository does not have html_url"))?;
            html_url.path_segments_mut().unwrap().push("releases");
            open::that(html_url.as_str())?;
        }
    }
    Ok(())
}

async fn create_asset(
    repo: &RepoName,
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
    let id = find_release(repo, api, &release)
        .await?
        .id
        .ok_or_else(|| eyre::eyre!("release does not have id"))?;
    let query = RepoCreateReleaseAttachmentQuery {
        name: Some(asset.to_owned()),
    };
    api.repo_create_release_attachment(
        repo.owner(),
        repo.name(),
        id,
        tokio::fs::read(file).await?,
        query,
    )
    .await?;

    Ok(())
}

async fn delete_asset(
    repo: &RepoName,
    api: &Forgejo,
    release: String,
    asset: String,
) -> eyre::Result<()> {
    let release = find_release(repo, api, &release).await?;
    let assets = release
        .assets
        .as_ref()
        .ok_or_else(|| eyre::eyre!("release does not have assets"))?;
    let asset = assets
        .iter()
        .find(|a| a.name.as_ref() == Some(&asset))
        .ok_or_else(|| eyre!("asset not found"))?;
    let release_id = release
        .id
        .ok_or_else(|| eyre::eyre!("release does not have id"))?;
    let asset_id = asset
        .id
        .ok_or_else(|| eyre::eyre!("asset does not have id"))?;
    api.repo_delete_release_attachment(repo.owner(), repo.name(), release_id, asset_id)
        .await?;
    Ok(())
}

async fn download_asset(
    repo: &RepoName,
    api: &Forgejo,
    release: String,
    asset: String,
    output: Option<std::path::PathBuf>,
) -> eyre::Result<()> {
    let release = find_release(repo, api, &release).await?;
    let file = match &*asset {
        "source.zip" => {
            let tag_name = release
                .tag_name
                .as_ref()
                .ok_or_else(|| eyre::eyre!("release does not have tag_name"))?;
            api.repo_get_archive(repo.owner(), repo.name(), &format!("{}.zip", tag_name))
                .await?
        }
        "source.tar.gz" => {
            let tag_name = release
                .tag_name
                .as_ref()
                .ok_or_else(|| eyre::eyre!("release does not have tag_name"))?;
            api.repo_get_archive(repo.owner(), repo.name(), &format!("{}.tar.gz", tag_name))
                .await?
        }
        name => {
            let assets = release
                .assets
                .as_ref()
                .ok_or_else(|| eyre::eyre!("release does not have assets"))?;
            let asset = assets
                .iter()
                .find(|a| a.name.as_deref() == Some(name))
                .ok_or_else(|| eyre!("asset not found"))?;
            let release_id = release
                .id
                .ok_or_else(|| eyre::eyre!("release does not have id"))?;
            let asset_id = asset
                .id
                .ok_or_else(|| eyre::eyre!("asset does not have id"))?;
            api.download_release_attachment(repo.owner(), repo.name(), release_id, asset_id)
                .await?
                .to_vec()
        }
    };
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
    repo: &RepoName,
    api: &Forgejo,
    name: &str,
) -> eyre::Result<forgejo_api::structs::Release> {
    let query = RepoListReleasesQuery {
        draft: None,
        pre_release: None,
        page: None,
        limit: None,
    };
    let mut releases = api
        .repo_list_releases(repo.owner(), repo.name(), query)
        .await?;
    let idx = releases
        .iter()
        .position(|r| r.name.as_deref() == Some(name))
        .ok_or_else(|| eyre!("release not found"))?;
    Ok(releases.swap_remove(idx))
}

async fn delete_release(
    repo: &RepoName,
    api: &Forgejo,
    name: String,
    by_tag: bool,
) -> eyre::Result<()> {
    if by_tag {
        api.repo_delete_release_by_tag(repo.owner(), repo.name(), &name)
            .await?;
    } else {
        let id = find_release(repo, api, &name)
            .await?
            .id
            .ok_or_else(|| eyre::eyre!("release does not have id"))?;
        api.repo_delete_release(repo.owner(), repo.name(), id)
            .await?;
    }
    Ok(())
}
