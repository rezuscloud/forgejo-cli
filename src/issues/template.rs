use eyre::OptionExt;
use forgejo_api::Forgejo;

pub mod yaml;

use crate::repo::RepoName;

pub struct MarkdownTemplate {
    pub labels: Option<Vec<String>>,
    pub body: String,
}

impl MarkdownTemplate {
    pub fn new(md: &str) -> eyre::Result<Self> {
        let md_without_start = md
            .strip_prefix("---\n")
            .or_else(|| md.strip_prefix("---\r\n"));
        let (front_matter, body) = md_without_start
            .and_then(|md| md.split_once("\n---\n"))
            .or_else(|| md_without_start.and_then(|md| md.split_once("\r\n---\r\n")))
            .ok_or_eyre("no front matter")?;

        #[derive(serde::Deserialize)]
        struct TemplateMetadata {
            labels: Option<Vec<String>>,
        }

        let metadata = serde_saphyr::from_str::<TemplateMetadata>(front_matter)?;

        Ok(Self {
            labels: metadata.labels,
            body: body.to_owned(),
        })
    }
}

pub async fn get_template_file(
    repo: &RepoName,
    api: &Forgejo,
    name: &str,
) -> eyre::Result<(Vec<u8>, bool)> {
    const DIRS: [&str; 8] = [
        ".forgejo/issue_template",
        ".forgejo/ISSUE_TEMPLATE",
        ".gitea/issue_template",
        ".gitea/ISSUE_TEMPLATE",
        ".github/issue_template",
        ".github/ISSUE_TEMPLATE",
        "docs/issue_template",
        "docs/ISSUE_TEMPLATE",
    ];
    const EXTS: [&str; 3] = ["md", "yml", "yaml"];
    let query = forgejo_api::structs::RepoGetRawFileQuery { r#ref: None };
    for dir in DIRS {
        for ext in EXTS {
            let path = format!("{dir}/{name}.{ext}");
            let file = api
                .repo_get_raw_file(repo.owner(), repo.name(), &path, query.clone())
                .await;
            match file {
                Ok(file) => {
                    let is_yaml = matches!(ext, "yml" | "yaml");
                    return Ok((file, is_yaml));
                }
                Err(forgejo_api::ForgejoError::ApiError(status, ..))
                    if status == hyper::http::StatusCode::NOT_FOUND =>
                {
                    ()
                }
                Err(e) => return Err(e.into()),
            }
        }
    }
    eyre::bail!("Could not find template '{name}'");
}
