use std::collections::BTreeMap;
use std::str::FromStr;

use clap::{Args, Subcommand};
use eyre::{eyre, Context, OptionExt};
use forgejo_api::structs::{
    Comment, CreateIssueCommentOption, CreateIssueOption, EditIssueOption, IssueGetCommentsQuery,
};
use forgejo_api::Forgejo;

use crate::repo::{RepoArg, RepoInfo, RepoName};

#[derive(Args, Clone, Debug)]
pub struct IssueCommand {
    /// The local git remote that points to the repo to operate on.
    #[clap(long, short = 'R')]
    remote: Option<String>,
    #[clap(subcommand)]
    command: IssueSubcommand,
}

#[derive(Subcommand, Clone, Debug)]
pub enum IssueSubcommand {
    /// Create a new issue on a repo
    Create {
        /// Title of the issue
        title: Option<String>,
        /// The text body of the issue
        ///
        /// Leaving this out will open your editor.
        #[clap(long, conflicts_with = "template")]
        body: Option<String>,
        /// The template to use when creating an issue
        ///
        /// If the repo has disabled blank issues, this flag is required.
        #[clap(long)]
        template: Option<String>,
        /// Don't use a template for this issue.
        ///
        /// If the repo has disabled blank issues, this will fail.
        #[clap(long, conflicts_with = "template")]
        no_template: bool,
        /// The repo to create this issue on
        #[clap(long, short)]
        repo: Option<RepoArg>,
        /// Open the PR creation page in your web browser
        #[clap(long)]
        web: bool,
    },
    /// Edit an issue
    Edit {
        issue: IssueId,
        #[clap(subcommand)]
        command: EditCommand,
    },
    /// Add a comment on an issue
    Comment {
        issue: IssueId,
        body: Option<String>,
    },
    /// Close an issue
    Close {
        issue: IssueId,
        /// A comment to leave on the issue before closing it
        #[clap(long, short)]
        with_msg: Option<Option<String>>,
    },
    /// Search for an issue in a repo
    Search {
        #[clap(long, short)]
        repo: Option<RepoArg>,
        query: Option<String>,
        #[clap(long, short)]
        labels: Option<String>,
        #[clap(long, short)]
        creator: Option<String>,
        #[clap(long, short)]
        assignee: Option<String>,
        #[clap(long, short)]
        state: Option<State>,
    },
    /// View an issue's info
    View {
        id: IssueId,
        #[clap(subcommand)]
        command: Option<ViewCommand>,
    },
    /// Open an issue in your browser
    Browse { id: IssueId },
}

#[derive(Clone, Debug)]
pub struct IssueId {
    pub repo: Option<RepoArg>,
    pub number: u64,
}

impl FromStr for IssueId {
    type Err = IssueIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (repo, number) = match s.rsplit_once("#") {
            Some((repo, number)) => (Some(repo.parse::<RepoArg>()?), number),
            None => (None, s),
        };
        Ok(Self {
            repo,
            number: number.parse()?,
        })
    }
}

#[derive(Debug, Clone)]
pub enum IssueIdError {
    Repo(crate::repo::RepoArgError),
    Number(std::num::ParseIntError),
}

impl std::fmt::Display for IssueIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueIdError::Repo(e) => e.fmt(f),
            IssueIdError::Number(e) => e.fmt(f),
        }
    }
}

impl From<crate::repo::RepoArgError> for IssueIdError {
    fn from(value: crate::repo::RepoArgError) -> Self {
        Self::Repo(value)
    }
}

impl From<std::num::ParseIntError> for IssueIdError {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::Number(value)
    }
}

impl std::error::Error for IssueIdError {}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum State {
    Open,
    Closed,
}

impl From<State> for forgejo_api::structs::IssueListIssuesQueryState {
    fn from(value: State) -> Self {
        match value {
            State::Open => forgejo_api::structs::IssueListIssuesQueryState::Open,
            State::Closed => forgejo_api::structs::IssueListIssuesQueryState::Closed,
        }
    }
}

#[derive(Subcommand, Clone, Debug)]
pub enum EditCommand {
    /// Edit an issue's title
    Title { new_title: Option<String> },
    /// Edit an issue's text content
    Body { new_body: Option<String> },
    /// Edit a comment on an issue
    Comment {
        idx: usize,
        new_body: Option<String>,
    },
}

#[derive(Subcommand, Clone, Debug)]
pub enum ViewCommand {
    /// View an issue's title and body. The default
    Body,
    /// View a specific
    Comment { idx: usize },
    /// List every comment
    Comments,
}

impl IssueCommand {
    pub async fn run(self, keys: &mut crate::KeyInfo, host_name: Option<&str>) -> eyre::Result<()> {
        use IssueSubcommand::*;
        let repo = RepoInfo::get_current(host_name, self.repo(), self.remote.as_deref(), &keys)?;
        let api = keys.get_api(repo.host_url()).await?;
        let repo = repo.name().ok_or_else(|| self.no_repo_error())?;
        match self.command {
            Create {
                repo: _,
                title,
                body,
                template,
                no_template,
                web,
            } => create_issue(repo, &api, title, body, template, no_template, web).await?,
            View { id, command } => match command.unwrap_or(ViewCommand::Body) {
                ViewCommand::Body => view_issue(repo, &api, id.number).await?,
                ViewCommand::Comment { idx } => view_comment(repo, &api, id.number, idx).await?,
                ViewCommand::Comments => view_comments(repo, &api, id.number).await?,
            },
            Search {
                repo: _,
                query,
                labels,
                creator,
                assignee,
                state,
            } => view_issues(repo, &api, query, labels, creator, assignee, state).await?,
            Edit { issue, command } => match command {
                EditCommand::Title { new_title } => {
                    edit_title(repo, &api, issue.number, new_title).await?
                }
                EditCommand::Body { new_body } => {
                    edit_body(repo, &api, issue.number, new_body).await?
                }
                EditCommand::Comment { idx, new_body } => {
                    edit_comment(repo, &api, issue.number, idx, new_body).await?
                }
            },
            Close { issue, with_msg } => close_issue(repo, &api, issue.number, with_msg).await?,
            Browse { id } => browse_issue(repo, &api, id.number).await?,
            Comment { issue, body } => add_comment(repo, &api, issue.number, body).await?,
        }
        Ok(())
    }

    fn repo(&self) -> Option<&RepoArg> {
        use IssueSubcommand::*;
        match &self.command {
            Create { repo, .. } | Search { repo, .. } => repo.as_ref(),
            View { id: issue, .. }
            | Edit { issue, .. }
            | Close { issue, .. }
            | Comment { issue, .. }
            | Browse { id: issue, .. } => issue.repo.as_ref(),
        }
    }

    fn no_repo_error(&self) -> eyre::Error {
        use IssueSubcommand::*;
        match &self.command {
            Create { .. } | Search { .. } => {
                eyre::eyre!("can't figure what repo to access, try specifying with `--repo`")
            }
            View { id: issue, .. }
            | Edit { issue, .. }
            | Close { issue, .. }
            | Comment { issue, .. }
            | Browse { id: issue, .. } => eyre::eyre!(
                "can't figure out what repo to access, try specifying with `{{owner}}/{{repo}}#{}`",
                issue.number
            ),
        }
    }
}

struct MarkdownTemplate {
    labels: Option<Vec<String>>,
    body: String,
}

impl MarkdownTemplate {
    fn new(md: &str) -> eyre::Result<Self> {
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

macro_rules! bail_at {
    ($node:expr, $fmt_str:literal) => {
        bail_at!($node, $fmt_str, )
    };
    ($node:expr, $fmt_str:literal, $($fmt:tt),*) => {
        bail_at!(@$node.data.borrow(), $fmt_str, )
    };
    (@$node:expr, $fmt_str:literal) => {
        bail_at!(@$node, $fmt_str, )
    };
    (@$node:expr, $fmt_str:literal, $($fmt:tt),*) => {
        return Err(eyre::eyre!($fmt_str, $($fmt)*)).wrap_err_with(
            || eyre::eyre!("unexpected content on line {}", $node.sourcepos.start.line),
        )
    }
}

macro_rules! ensure_at {
    ($node:expr, $e:expr, $fmt_str:literal) => {
        ensure_at!($node, $e, $fmt_str, )
    };
    ($node:expr, $e:expr, $fmt_str:literal, $($fmt:tt),*) => {
        if !($e) {
            bail_at!($node, $fmt_str);
        }
    };
    (@$node:expr, $e:expr, $fmt_str:literal) => {
        ensure_at!(@$node, $e, $fmt_str, )
    };
    (@$node:expr, $e:expr, $fmt_str:literal, $($fmt:tt),*) => {
        if !($e) {
            bail_at!(@$node, $fmt_str);
        }
    }
}

macro_rules! require_node {
    ($iter:ident, $expected:expr, $fmt_str:literal) => {
        require_node!($iter, $expected, $fmt_str, )
    };
    ($iter:ident, $expected:expr, $fmt_str:literal, $($fmt:tt),*) => {
        {
            let node = $iter.next().ok_or_eyre("unexpected EOF")?;
            ensure_at!(node, node.data.borrow().value == $expected, $fmt_str, $($fmt,)*);
            node
        }
    };
}

const FIELD_REQUIRED: &str = "Required.";
const FIELD_NUMBER: &str = "Must be a number.";
const FIELD_REGEX: &str = "Must match the following regex: ";
const FIELD_AT_LEAST_ONE: &str = "Must select at least one option.";
const FIELD_AT_MOST_ONE: &str = "Must select at most one option.";
const FIELD_EXACTLY_ONE: &str = "Must select exactly one option.";
const FIELD_CHECKBOX_REQUIRED: &str = "(required) ";

#[derive(serde::Deserialize, Debug)]
struct YamlTemplate {
    labels: Option<Vec<String>>,
    body: Vec<TemplateItem>,
}

impl YamlTemplate {
    fn generate_form(&self) -> eyre::Result<String> {
        use comrak::nodes::{NodeList, NodeValue};

        let arena = &comrak::Arena::new();
        let mut options = comrak::Options::default();
        options.extension.strikethrough = true;
        options.extension.tasklist = true;
        options.render.unsafe_ = true;
        let options = &options;
        let output = arena.alloc(NodeValue::Document.into());
        for item in &self.body {
            if !item.visibility().form {
                continue;
            }
            match item {
                TemplateItem::Markdown { attributes, .. } => {
                    append_markdown(arena, output, &attributes.value, options);
                }
                TemplateItem::TextArea {
                    attributes,
                    validations,
                    ..
                } => {
                    append_header(arena, output, 3, &attributes.label);
                    if let Some(description) = &attributes.description {
                        append_markdown(arena, output, description, options);
                    }

                    if validations.required {
                        append_node(
                            arena,
                            output,
                            NodeValue::Raw(format!("{FIELD_REQUIRED}\n\n")),
                        );
                    }

                    // Comrak ignores the `fence_char` and `fence_length` fields
                    // when formatting to markdown, so a Raw node is needed instead.
                    append_node(
                        arena,
                        output,
                        NodeValue::Raw(format!(
                            "~~~~~ {info}\n{literal}\n~~~~~\n\n",
                            info = attributes.render.as_deref().unwrap_or("markdown"),
                            literal = attributes.value,
                        )),
                    );
                }
                TemplateItem::Input {
                    attributes,
                    validations,
                    ..
                } => {
                    append_header(arena, output, 3, &attributes.label);
                    if let Some(description) = &attributes.description {
                        append_markdown(arena, output, description, options);
                    }

                    if validations.required {
                        append_node(
                            arena,
                            output,
                            NodeValue::Raw(format!("{FIELD_REQUIRED}\n\n")),
                        );
                    }

                    if validations.is_number {
                        append_node(arena, output, NodeValue::Raw(format!("{FIELD_NUMBER}\n\n")));
                    }

                    if let Some(regex) = &validations.regex {
                        append_node(
                            arena,
                            output,
                            NodeValue::Raw(format!("{FIELD_REGEX}`{regex}`.\n\n")),
                        );
                    }

                    let textarea_blockquote = append_node(arena, output, NodeValue::BlockQuote);
                    append_markdown(arena, textarea_blockquote, &attributes.value, options);
                }
                TemplateItem::Dropdown {
                    attributes,
                    validations,
                    ..
                } => {
                    append_header(arena, output, 3, &attributes.label);
                    if let Some(description) = &attributes.description {
                        append_markdown(arena, output, description, options);
                    }

                    let requirements = match (validations.required, attributes.multiple) {
                        (true, true) => Some(FIELD_AT_LEAST_ONE),
                        (true, false) => Some(FIELD_EXACTLY_ONE),
                        (false, true) => None,
                        (false, false) => Some(FIELD_AT_MOST_ONE),
                    };
                    if let Some(requirements) = requirements {
                        append_node(arena, output, NodeValue::Raw(format!("{requirements}\n\n")));
                    }

                    let list_cfg = NodeList {
                        tight: true,
                        ..Default::default()
                    };
                    let list = append_node(arena, output, NodeValue::List(list_cfg));
                    for list_option in &attributes.options {
                        let list_item = append_node(arena, list, NodeValue::TaskItem(None));
                        append_markdown(arena, list_item, &list_option, options);
                    }
                }
                TemplateItem::Checkboxes { attributes, .. } => {
                    append_header(arena, output, 3, &attributes.label);
                    if let Some(description) = &attributes.description {
                        append_markdown(arena, output, description, options);
                    }

                    let list_cfg = NodeList {
                        tight: true,
                        ..Default::default()
                    };
                    let list = append_node(arena, output, NodeValue::List(list_cfg));
                    for list_option in &attributes.options {
                        if list_option.visible.form {
                            let list_item = append_node(arena, list, NodeValue::TaskItem(None));
                            let label = if list_option.required {
                                &format!("{FIELD_CHECKBOX_REQUIRED}{}", list_option.label)
                            } else {
                                &list_option.label
                            };
                            append_markdown(arena, list_item, label, options);
                        }
                    }
                }
            }
        }
        let mut output_buf = Vec::new();
        comrak::format_commonmark(output, options, &mut output_buf)?;
        let output_str = String::from_utf8(output_buf)?;
        Ok(output_str)
    }

    fn parse_form<'a, 's: 'a>(&'s self, form: &str) -> eyre::Result<Vec<Option<FieldValue>>> {
        use comrak::nodes::{NodeCodeBlock, NodeList, NodeValue};

        let arena = &comrak::Arena::new();
        let mut options = comrak::Options::default();
        options.extension.strikethrough = true;
        options.extension.tasklist = true;
        options.render.unsafe_ = true;
        let options = &options;

        let form = comrak::parse_document(arena, form, options);
        let mut form_iter = form.children();

        let mut output = Vec::new();

        for item in &self.body {
            if !item.visibility().form {
                output.push(None);
                continue;
            }
            if let Some(header) = item.header() {
                validate_header(arena, &mut form_iter, 3, header, options)?;
            }
            if let Some(description) = item.description() {
                validate_description(arena, &mut form_iter, description, options)?;
            }
            match item {
                // this is already covered by the description validation
                TemplateItem::Markdown { .. } => {
                    output.push(None);
                }

                TemplateItem::TextArea {
                    attributes,
                    validations,
                    ..
                } => {
                    if validations.required {
                        validate_description(arena, &mut form_iter, FIELD_REQUIRED, options)?;
                    }

                    let node = form_iter.next().ok_or_eyre("unexpected EOF")?;

                    let render = attributes.render.as_deref().unwrap_or("markdown");

                    let node_data = node.data.borrow();
                    match &node_data.value {
                        NodeValue::CodeBlock(NodeCodeBlock {
                            fenced: true,
                            fence_char: b'~',
                            // fence_length: Intentionally not checked for
                            // in case the user needs to extend the fence
                            info,
                            literal,
                            ..
                        }) if info == render => {
                            ensure_at!(
                                @node_data,
                                !(validations.required && literal.is_empty()),
                                "missing required field",
                            );
                            output.push(Some(FieldValue::Input(literal.to_owned())));
                        }

                        _ => bail_at!(@node_data, "expected `{render}` codeblock"),
                    }
                }

                TemplateItem::Input { validations, .. } => {
                    if validations.required {
                        validate_description(arena, &mut form_iter, FIELD_REQUIRED, options)?;
                    }
                    if validations.is_number {
                        validate_description(arena, &mut form_iter, FIELD_NUMBER, options)?;
                    }
                    if let Some(regex) = &validations.regex {
                        validate_description(
                            arena,
                            &mut form_iter,
                            &format!("{FIELD_REGEX}`{regex}`."),
                            options,
                        )?;
                    }

                    let field =
                        require_node!(form_iter, NodeValue::BlockQuote, "expected block quote");
                    ensure_at!(
                        field,
                        !(validations.required && field.children().next().is_none()),
                        "missing required field",
                    );
                    ensure_at!(
                        field,
                        field
                            .first_child()
                            .zip(field.last_child())
                            .is_none_or(|(a, b)| a.same_node(b)),
                        "cannot submit multiline value",
                    );

                    let new_doc = arena.alloc(NodeValue::Document.into());
                    for child in field.children() {
                        new_doc.append(child);
                    }

                    let mut body = Vec::new();
                    comrak::format_commonmark(new_doc, options, &mut body)?;
                    let mut body = String::from_utf8(body)?;
                    if body.ends_with("\r\n") {
                        body.pop();
                        body.pop();
                    } else if body.ends_with("\n") {
                        body.pop();
                    }
                    if validations.is_number {
                        ensure_at!(
                            field,
                            body.trim().parse::<i64>().is_ok()
                                || body.trim().parse::<f64>().is_ok(),
                            "submitted value must be a number",
                        );
                    }
                    if let Some(regex_str) = &validations.regex {
                        let regex = regex::Regex::new(regex_str).wrap_err("invalid regex")?;
                        ensure_at!(
                            field,
                            regex.is_match(&body),
                            "must match regex \"{regex_str}\""
                        );
                    }
                    output.push(Some(FieldValue::Input(body)));
                }

                TemplateItem::Dropdown {
                    attributes,
                    validations,
                    ..
                } => {
                    let requirements = match (validations.required, attributes.multiple) {
                        (true, true) => Some(FIELD_AT_LEAST_ONE),
                        (true, false) => Some(FIELD_EXACTLY_ONE),
                        (false, true) => None,
                        (false, false) => Some(FIELD_AT_MOST_ONE),
                    };
                    if let Some(requirements) = requirements {
                        validate_description(arena, &mut form_iter, requirements, options)?;
                    }

                    let list_cfg = NodeList {
                        is_task_list: true,
                        tight: true,
                        padding: 2,
                        start: 1,
                        bullet_char: b'-',
                        ..Default::default()
                    };
                    let list = require_node!(form_iter, NodeValue::List(list_cfg), "expected list");

                    let mut ticked = Vec::new();

                    let mut children = list.children();
                    let mut is_first_ticked = true;
                    for option in &attributes.options {
                        let child = children.next().ok_or_eyre("unexpected end of list")?;
                        let child_data = child.data.borrow();
                        let is_ticked = match child_data.value {
                            NodeValue::TaskItem(fill_char) => fill_char.is_some(),
                            _ => bail_at!(@child_data, "expected task list"),
                        };
                        validate_contents(arena, child, option, options).wrap_err("dropdown")?;

                        if is_ticked {
                            eyre::ensure!(
                                is_first_ticked || attributes.multiple,
                                "only one item can be selected"
                            );
                            is_first_ticked = false;
                        }

                        ticked.push(is_ticked);
                    }
                    if let Some(extra_child) = children.next() {
                        bail_at!(extra_child, "unexpected extra item");
                    }
                    ensure_at!(
                        list,
                        !(validations.required && is_first_ticked),
                        "at least one item must be checked"
                    );
                    output.push(Some(FieldValue::Checkboxes(ticked)));
                }

                TemplateItem::Checkboxes { attributes, .. } => {
                    let list_cfg = NodeList {
                        is_task_list: true,
                        tight: true,
                        padding: 2,
                        start: 1,
                        bullet_char: b'-',
                        ..Default::default()
                    };
                    let list = require_node!(form_iter, NodeValue::List(list_cfg), "expected list");

                    let mut ticked = Vec::new();

                    let mut children = list.children();
                    for option in &attributes.options {
                        if option.visible.form {
                            let child = children.next().ok_or_eyre("unexpected end of list")?;
                            let child_data = child.data.borrow();
                            let is_ticked = match child_data.value {
                                NodeValue::TaskItem(fill_char) => fill_char.is_some(),
                                _ => bail_at!(@child_data, "expected task list"),
                            };
                            let label = if option.required {
                                &format!("{FIELD_CHECKBOX_REQUIRED}{}", option.label)
                            } else {
                                &option.label
                            };
                            validate_contents(arena, child, label, options).wrap_err("checkbox")?;
                            ensure_at!(@child_data, is_ticked || !option.required, "option is required");

                            ticked.push(is_ticked);
                        } else {
                            ticked.push(false);
                        }
                    }
                    if let Some(extra_child) = children.next() {
                        bail_at!(extra_child, "unexpected extra item");
                    }
                    output.push(Some(FieldValue::Checkboxes(ticked)));
                }
            }
        }

        Ok(output)
    }

    fn generate_content<'a, 's: 'a>(
        &'s self,
        form: Vec<Option<FieldValue>>,
    ) -> eyre::Result<String> {
        use comrak::nodes::{NodeCodeBlock, NodeList, NodeValue};

        let arena = &comrak::Arena::new();
        let mut options = comrak::Options::default();
        options.extension.strikethrough = true;
        options.extension.tasklist = true;
        options.render.unsafe_ = true;
        let options = &options;

        let output = arena.alloc(NodeValue::Document.into());
        for (item, field_value) in self.body.iter().zip(form.into_iter()) {
            if !item.visibility().content {
                continue;
            }
            match item {
                TemplateItem::Markdown { attributes, .. } => {
                    append_markdown(arena, output, &attributes.value, options);
                }
                TemplateItem::TextArea { attributes, .. } => {
                    append_header(arena, output, 3, &attributes.label);
                    match field_value {
                        Some(FieldValue::Input(body)) => {
                            if body.trim().is_empty() {
                                append_node(
                                    arena,
                                    output,
                                    NodeValue::Raw("_No response_\n\n".into()),
                                );
                            } else if let Some(render) = &attributes.render {
                                append_node(
                                    arena,
                                    output,
                                    NodeValue::CodeBlock(NodeCodeBlock {
                                        fenced: true,
                                        info: render.into(),
                                        literal: body,
                                        ..Default::default()
                                    }),
                                );
                            } else {
                                append_node(arena, output, NodeValue::Raw(body));
                                append_node(arena, output, NodeValue::Raw("\n".into()));
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                TemplateItem::Input { attributes, .. } => {
                    append_header(arena, output, 3, &attributes.label);
                    let value = match field_value {
                        Some(FieldValue::Input(body)) if !body.trim().is_empty() => body,
                        Some(FieldValue::Checkboxes(_)) => unreachable!(),
                        _ => "_No response_".into(),
                    };
                    append_node(arena, output, NodeValue::Raw(value));
                    append_node(arena, output, NodeValue::Raw("\n\n".into()));
                }
                TemplateItem::Dropdown { attributes, .. } => {
                    append_header(arena, output, 3, &attributes.label);
                    let ticked = match field_value {
                        Some(FieldValue::Input(_)) => unreachable!(),
                        Some(FieldValue::Checkboxes(ticked)) => ticked,
                        None => vec![],
                    };
                    let p = append_node(arena, output, NodeValue::Paragraph);
                    let ticked_iter = ticked.into_iter().chain(std::iter::repeat(false));
                    let mut is_first_ticked = true;
                    for (option, is_ticked) in attributes.options.iter().zip(ticked_iter) {
                        if is_ticked {
                            if !is_first_ticked {
                                append_node(arena, p, NodeValue::Raw(", ".into()));
                            }
                            append_node(arena, p, NodeValue::Raw(option.clone()));
                            is_first_ticked = false;
                        }
                    }
                    if is_first_ticked {
                        append_node(arena, output, NodeValue::Raw("_No response_\n\n".into()));
                    }
                }
                TemplateItem::Checkboxes { attributes, .. } => {
                    append_header(arena, output, 3, &attributes.label);
                    let ticked = match field_value {
                        Some(FieldValue::Input(_)) => unreachable!(),
                        Some(FieldValue::Checkboxes(ticked)) => ticked,
                        None => vec![],
                    };
                    let list_cfg = NodeList {
                        tight: true,
                        ..Default::default()
                    };
                    let list = append_node(arena, output, NodeValue::List(list_cfg));
                    let ticked_iter = ticked.into_iter().chain(std::iter::repeat(false));
                    for (option, is_ticked) in attributes.options.iter().zip(ticked_iter) {
                        if option.visible.content {
                            let list_item = append_node(
                                arena,
                                list,
                                NodeValue::TaskItem(is_ticked.then_some('x')),
                            );
                            append_node(arena, list_item, NodeValue::Raw(option.label.clone()));
                        }
                    }
                }
            }
        }
        let mut output_buf = Vec::new();
        comrak::format_commonmark(output, options, &mut output_buf)?;
        let output_str = String::from_utf8(output_buf)?;
        Ok(output_str)
    }
}

fn append_node<'a>(
    arena: &'a comrak::Arena<comrak::nodes::AstNode<'a>>,
    parent: &'a comrak::nodes::AstNode<'a>,
    value: comrak::nodes::NodeValue,
) -> &'a comrak::nodes::AstNode<'a> {
    let node = arena.alloc(value.into());
    parent.append(node);
    node
}

fn append_markdown<'a>(
    arena: &'a comrak::Arena<comrak::nodes::AstNode<'a>>,
    parent: &'a comrak::nodes::AstNode<'a>,
    md: &str,
    options: &comrak::Options<'_>,
) {
    let parsed = comrak::parse_document(arena, md, options);
    for child in parsed.children() {
        parent.append(child);
    }
}

fn validate_contents<'a>(
    arena: &'a comrak::Arena<comrak::nodes::AstNode<'a>>,
    parent: &'a comrak::nodes::AstNode<'a>,
    md: &str,
    options: &comrak::Options<'_>,
) -> eyre::Result<()> {
    let parsed = comrak::parse_document(arena, md, options);
    ensure_at!(parent, children_eq(parent, parsed), "modified content",);
    Ok(())
}

fn validate_description<'a>(
    arena: &'a comrak::Arena<comrak::nodes::AstNode<'a>>,
    form: &mut comrak::arena_tree::Children<'a, std::cell::RefCell<comrak::nodes::Ast>>,
    md: &str,
    options: &comrak::Options<'_>,
) -> eyre::Result<()> {
    let parsed = comrak::parse_document(arena, md, options);
    for a in parsed.children() {
        let b = form.next().ok_or_eyre("unexpected EOF")?;
        ensure_at!(b, nodes_eq(a, b), "modified content",);
    }
    Ok(())
}

fn append_header<'a>(
    arena: &'a comrak::Arena<comrak::nodes::AstNode<'a>>,
    parent: &'a comrak::nodes::AstNode<'a>,
    level: u8,
    content: &str,
) {
    use comrak::nodes::{NodeHeading, NodeValue};
    let header = append_node(
        arena,
        parent,
        NodeValue::Heading(NodeHeading {
            level,
            setext: false,
        }),
    );
    append_node(arena, header, NodeValue::Raw(content.into()));
}

fn validate_header<'a>(
    arena: &'a comrak::Arena<comrak::nodes::AstNode<'a>>,
    form: &mut comrak::arena_tree::Children<'a, std::cell::RefCell<comrak::nodes::Ast>>,
    level: u8,
    content: &str,
    options: &comrak::Options<'_>,
) -> eyre::Result<()> {
    use comrak::nodes::{NodeHeading, NodeValue};

    let form_heading = require_node!(
        form,
        NodeValue::Heading(NodeHeading {
            level,
            setext: false
        }),
        "expected header"
    );
    let parsed = comrak::parse_document(arena, content, options);
    let parsed_inline = parsed.first_child().ok_or_eyre("invalid label")?;
    ensure_at!(
        form_heading,
        children_eq(form_heading, parsed_inline),
        "expected header"
    );
    Ok(())
}

fn nodes_eq<'a>(a: &'a comrak::nodes::AstNode<'a>, b: &'a comrak::nodes::AstNode<'a>) -> bool {
    if a.data.borrow().value != b.data.borrow().value {
        return false;
    }
    children_eq(a, b)
}

fn children_eq<'a>(a: &'a comrak::nodes::AstNode<'a>, b: &'a comrak::nodes::AstNode<'a>) -> bool {
    let mut iter_a = a.children();
    let mut iter_b = b.children();
    loop {
        match (iter_a.next(), iter_b.next()) {
            (Some(a), Some(b)) => {
                if !nodes_eq(a, b) {
                    return false;
                }
            }
            (Some(_), None) | (None, Some(_)) => return false,
            (None, None) => break,
        }
    }
    true
}

enum FieldValue {
    Input(String),
    Checkboxes(Vec<bool>),
}

#[derive(serde::Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
enum TemplateItem {
    Markdown {
        attributes: MarkdownItemAttributes,
        #[serde(default = "TemplateVisibility::form")]
        visible: TemplateVisibility,
    },
    TextArea {
        attributes: TextAreaItemAttributes,
        #[serde(default)]
        validations: RequiredValidation,
        #[serde(default = "TemplateVisibility::both")]
        visible: TemplateVisibility,
    },
    Input {
        attributes: InputItemAttributes,
        #[serde(default)]
        validations: InputItemValidation,
        #[serde(default = "TemplateVisibility::both")]
        visible: TemplateVisibility,
    },
    Dropdown {
        attributes: DropdownItemAttributes,
        #[serde(default)]
        validations: RequiredValidation,
        #[serde(default = "TemplateVisibility::both")]
        visible: TemplateVisibility,
    },
    Checkboxes {
        attributes: CheckboxesItemAttributes,
        #[serde(default = "TemplateVisibility::both")]
        visible: TemplateVisibility,
    },
}

impl TemplateItem {
    fn visibility(&self) -> TemplateVisibility {
        match self {
            TemplateItem::Markdown { visible, .. }
            | TemplateItem::TextArea { visible, .. }
            | TemplateItem::Input { visible, .. }
            | TemplateItem::Dropdown { visible, .. }
            | TemplateItem::Checkboxes { visible, .. } => *visible,
        }
    }

    fn header(&self) -> Option<&str> {
        match self {
            TemplateItem::Markdown { .. } => None,
            TemplateItem::TextArea { attributes, .. } => Some(&attributes.label),
            TemplateItem::Input { attributes, .. } => Some(&attributes.label),
            TemplateItem::Dropdown { attributes, .. } => Some(&attributes.label),
            TemplateItem::Checkboxes { attributes, .. } => Some(&attributes.label),
        }
    }

    fn description(&self) -> Option<&str> {
        match self {
            TemplateItem::Markdown { attributes, .. } => Some(&attributes.value),
            TemplateItem::TextArea { attributes, .. } => attributes.description.as_deref(),
            TemplateItem::Input { attributes, .. } => attributes.description.as_deref(),
            TemplateItem::Dropdown { attributes, .. } => attributes.description.as_deref(),
            TemplateItem::Checkboxes { attributes, .. } => attributes.description.as_deref(),
        }
    }
}

#[derive(serde::Deserialize, Debug)]
struct MarkdownItemAttributes {
    value: String,
}

#[derive(serde::Deserialize, Debug)]
struct TextAreaItemAttributes {
    label: String,
    description: Option<String>,
    #[serde(default)]
    value: String,
    render: Option<String>,
}

#[derive(serde::Deserialize, Debug, Default)]
struct RequiredValidation {
    #[serde(default)]
    required: bool,
}

#[derive(serde::Deserialize, Debug)]
struct InputItemAttributes {
    label: String,
    description: Option<String>,
    #[serde(default)]
    value: String,
}

#[derive(serde::Deserialize, Debug, Default)]
struct InputItemValidation {
    #[serde(default)]
    required: bool,
    #[serde(default)]
    is_number: bool,
    regex: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
struct DropdownItemAttributes {
    label: String,
    description: Option<String>,
    #[serde(default)]
    multiple: bool,
    options: Vec<String>,
}

#[derive(serde::Deserialize, Debug)]
struct CheckboxesItemAttributes {
    label: String,
    description: Option<String>,
    options: Vec<CheckboxOption>,
}

#[derive(serde::Deserialize, Debug)]
struct CheckboxOption {
    label: String,
    #[serde(default)]
    required: bool,
    #[serde(default = "TemplateVisibility::both")]
    visible: TemplateVisibility,
}

#[derive(serde::Deserialize, Debug, Clone, Copy)]
#[serde(from = "Vec<String>")]
struct TemplateVisibility {
    form: bool,
    content: bool,
}

impl TemplateVisibility {
    fn none() -> Self {
        Self {
            form: false,
            content: false,
        }
    }
    fn form() -> Self {
        Self {
            form: true,
            content: false,
        }
    }
    fn both() -> Self {
        Self {
            form: true,
            content: true,
        }
    }
}

impl From<Vec<String>> for TemplateVisibility {
    fn from(vec: Vec<String>) -> Self {
        vec.iter().fold(Self::none(), |mut v, s| {
            match &**s {
                "content" => v.content = true,
                "form" => v.form = true,
                _ => (),
            };
            v
        })
    }
}

async fn get_template_file(
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

async fn label_names_to_ids(
    repo: &RepoName,
    api: &Forgejo,
    names: Vec<String>,
) -> eyre::Result<Vec<i64>> {
    // convert from label names to label ids
    let mut all_labels = BTreeMap::new();
    for page_num in 1.. {
        let query = forgejo_api::structs::IssueListLabelsQuery {
            page: Some(page_num),
            limit: Some(50),
        };
        let (headers, page) = api
            .issue_list_labels(repo.owner(), repo.name(), query)
            .await?;
        let empty_page = page.is_empty();
        for label in page {
            let name = label.name.ok_or_eyre("label does not have name")?;
            let id = label.id.ok_or_eyre("label does not have name")?;
            all_labels.insert(name, id);
        }
        if empty_page
            || headers
                .x_total_count
                .is_none_or(|count| all_labels.len() >= count as usize)
        {
            break;
        }
    }
    Ok(names
        .into_iter()
        .filter_map(|name| all_labels.remove(&name))
        .collect())
}

async fn create_issue(
    repo: &RepoName,
    api: &Forgejo,
    title: Option<String>,
    body: Option<String>,
    template: Option<String>,
    no_template: bool,
    web: bool,
) -> eyre::Result<()> {
    match (title, web) {
        (Some(title), false) => {
            let blank_issues_enabled = api
                .repo_get_issue_config(repo.owner(), repo.name())
                .await
                .ok()
                .and_then(|cfg| cfg.blank_issues_enabled);
            let opts = if let Some(template_name) = template {
                eyre::ensure!(
                    blank_issues_enabled.is_some(),
                    "{}/{} does not have any issue templates",
                    repo.owner(),
                    repo.name()
                );
                let (template_file, is_yaml) = get_template_file(repo, api, &template_name).await?;
                let template_file = std::str::from_utf8(&template_file)?;
                if is_yaml {
                    let tmpl = serde_saphyr::from_str::<YamlTemplate>(template_file)?;

                    let mut form = tmpl.generate_form()?;
                    crate::editor(&mut form, Some("md")).await?;
                    let body = tmpl.generate_content(tmpl.parse_form(&form)?)?;

                    let labels = if let Some(labels) = tmpl.labels {
                        Some(label_names_to_ids(repo, api, labels).await?)
                    } else {
                        None
                    };

                    CreateIssueOption {
                        body: Some(body),
                        title,
                        assignee: None,
                        assignees: None,
                        closed: None,
                        due_date: None,
                        labels,
                        milestone: None,
                        r#ref: None,
                    }
                } else {
                    let mut tmpl = MarkdownTemplate::new(template_file)?;
                    crate::editor(&mut tmpl.body, Some("md")).await?;

                    let labels = if let Some(labels) = tmpl.labels {
                        Some(label_names_to_ids(repo, api, labels).await?)
                    } else {
                        None
                    };

                    CreateIssueOption {
                        body: Some(tmpl.body),
                        title,
                        assignee: None,
                        assignees: None,
                        closed: None,
                        due_date: None,
                        labels: labels,
                        milestone: None,
                        r#ref: None,
                    }
                }
            } else {
                eyre::ensure!(
                    blank_issues_enabled.unwrap_or(true),
                    "{}/{} requires using a template. \
                    Please choose one with `--template <NAME>`",
                    repo.owner(),
                    repo.name()
                );
                eyre::ensure!(
                    blank_issues_enabled.is_none() || no_template,
                    "{}/{} uses issue templates. \
                    Please choose one with `--template <NAME>`, \
                    or use `--no-template` to write one from scratch",
                    repo.owner(),
                    repo.name()
                );
                let body = match body {
                    Some(body) => body,
                    None => {
                        let mut body = String::new();
                        crate::editor(&mut body, Some("md")).await?;
                        body
                    }
                };
                CreateIssueOption {
                    body: Some(body),
                    title,
                    assignee: None,
                    assignees: None,
                    closed: None,
                    due_date: None,
                    labels: None,
                    milestone: None,
                    r#ref: None,
                }
            };
            let issue = api
                .issue_create_issue(repo.owner(), repo.name(), opts)
                .await?;
            let number = issue
                .number
                .ok_or_else(|| eyre::eyre!("issue does not have number"))?;
            let title = issue
                .title
                .as_ref()
                .ok_or_else(|| eyre::eyre!("issue does not have title"))?;
            eprintln!("created issue #{}: {}", number, title);
        }
        (None, true) => {
            let base_repo = api.repo_get(repo.owner(), repo.name()).await?;
            let mut issue_create_url = base_repo
                .html_url
                .clone()
                .ok_or_eyre("repo does not have html url")?;
            issue_create_url
                .path_segments_mut()
                .expect("invalid url")
                .extend(["issues", "new"]);
            open::that_detached(issue_create_url.as_str()).wrap_err("Failed to open URL")?;
        }
        (None, false) => {
            eyre::bail!("requires either issue title or --web flag")
        }
        (Some(_), true) => {
            eyre::bail!("issue title and --web flag are mutually exclusive")
        }
    }
    Ok(())
}

pub async fn view_issue(repo: &RepoName, api: &Forgejo, id: u64) -> eyre::Result<()> {
    let crate::SpecialRender {
        dash,

        bright_red,
        bright_green,
        yellow,
        dark_grey,
        white,
        reset,
        ..
    } = crate::special_render();

    let issue = api.issue_get_issue(repo.owner(), repo.name(), id).await?;

    // if it's a pull request, display it as one instead
    if issue.pull_request.is_some() {
        crate::prs::view_pr(repo, api, Some(id)).await?;
        return Ok(());
    }

    let title = issue
        .title
        .as_ref()
        .ok_or_else(|| eyre::eyre!("issue does not have title"))?;
    let user = issue
        .user
        .as_ref()
        .ok_or_else(|| eyre::eyre!("issue does not have creator"))?;
    let username = user
        .login
        .as_ref()
        .ok_or_else(|| eyre::eyre!("user does not have login"))?;
    let state = issue
        .state
        .ok_or_else(|| eyre::eyre!("pr does not have state"))?;
    let comments = issue.comments.unwrap_or_default();

    println!("{yellow}{title} {dark_grey}#{id}{reset}");
    print!("By {white}{username}{reset} {dash} ");

    use forgejo_api::structs::StateType;
    match state {
        StateType::Open => println!("{bright_green}Open{reset}"),
        StateType::Closed => println!("{bright_red}Closed{reset}"),
    };

    if let Some(body) = &issue.body {
        if !body.is_empty() {
            println!();
            println!("{}", crate::markdown(body));
        }
    }
    println!();

    if comments == 1 {
        println!("1 comment");
    } else {
        println!("{comments} comments");
    }
    Ok(())
}
async fn view_issues(
    repo: &RepoName,
    api: &Forgejo,
    query_str: Option<String>,
    labels: Option<String>,
    creator: Option<String>,
    assignee: Option<String>,
    state: Option<State>,
) -> eyre::Result<()> {
    let labels = labels
        .map(|s| s.split(',').map(|s| s.to_string()).collect::<Vec<_>>())
        .unwrap_or_default();
    let mut query = forgejo_api::structs::IssueListIssuesQuery {
        q: query_str,
        labels: Some(labels.join(",")),
        created_by: creator,
        assigned_by: assignee,
        state: state.map(|s| s.into()),
        r#type: None,
        milestones: None,
        since: None,
        before: None,
        mentioned_by: None,
        page: None,
        limit: None,
    };
    let mut issues = Vec::new();
    for page_idx in 1.. {
        query.page = Some(page_idx);
        let (headers, page) = api
            .issue_list_issues(repo.owner(), repo.name(), query.clone())
            .await?;
        issues.extend(page);
        if issues.len() >= headers.x_total_count.unwrap_or_default() as usize {
            break;
        }
    }
    if issues.len() == 1 {
        println!("1 issue");
    } else {
        println!("{} issues", issues.len());
    }
    for issue in issues {
        let number = issue
            .number
            .ok_or_else(|| eyre::eyre!("issue does not have number"))?;
        let title = issue
            .title
            .as_ref()
            .ok_or_else(|| eyre::eyre!("issue does not have title"))?;
        let user = issue
            .user
            .as_ref()
            .ok_or_else(|| eyre::eyre!("issue does not have creator"))?;
        let username = user
            .login
            .as_ref()
            .ok_or_else(|| eyre::eyre!("user does not have login"))?;
        println!("#{}: {} (by {})", number, title, username);
    }
    Ok(())
}

pub async fn view_comment(repo: &RepoName, api: &Forgejo, id: u64, idx: usize) -> eyre::Result<()> {
    let query = IssueGetCommentsQuery {
        since: None,
        before: None,
    };
    let (_, comments) = api
        .issue_get_comments(repo.owner(), repo.name(), id, query)
        .await?;
    let comment = comments
        .get(idx)
        .ok_or_else(|| eyre!("comment {idx} doesn't exist"))?;
    print_comment(comment)?;
    Ok(())
}

pub async fn view_comments(repo: &RepoName, api: &Forgejo, id: u64) -> eyre::Result<()> {
    let query = IssueGetCommentsQuery {
        since: None,
        before: None,
    };
    let (_, comments) = api
        .issue_get_comments(repo.owner(), repo.name(), id, query)
        .await?;
    for comment in comments {
        print_comment(&comment)?;
        println!();
    }
    Ok(())
}

fn print_comment(comment: &Comment) -> eyre::Result<()> {
    let body = comment
        .body
        .as_ref()
        .ok_or_else(|| eyre::eyre!("comment does not have body"))?;
    let user = comment
        .user
        .as_ref()
        .ok_or_else(|| eyre::eyre!("comment does not have user"))?;
    let name = user.full_name.as_deref().filter(|name| !name.is_empty());
    let username = user
        .login
        .as_ref()
        .ok_or_else(|| eyre::eyre!("user does not have login"))?;

    let crate::SpecialRender {
        bold,
        bright_cyan,
        dark_grey,
        reset,
        ..
    } = crate::special_render();
    if let Some(name) = name {
        println!("{bold}{bright_cyan}{name}{reset} {dark_grey}({username}){reset} said:");
    } else {
        println!("{bold}{bright_cyan}{username}{reset} said:");
    }
    println!("{}", crate::markdown(body));
    let assets = comment
        .assets
        .as_ref()
        .ok_or_else(|| eyre::eyre!("comment does not have assets"))?;
    if !assets.is_empty() {
        println!("({} attachments)", assets.len());
    }
    Ok(())
}

pub async fn browse_issue(repo: &RepoName, api: &Forgejo, id: u64) -> eyre::Result<()> {
    let issue = api.issue_get_issue(repo.owner(), repo.name(), id).await?;
    let html_url = issue
        .html_url
        .as_ref()
        .ok_or_else(|| eyre::eyre!("issue does not have html_url"))?;
    open::that_detached(html_url.as_str()).wrap_err("Failed to open URL")?;
    Ok(())
}

pub async fn add_comment(
    repo: &RepoName,
    api: &Forgejo,
    issue: u64,
    body: Option<String>,
) -> eyre::Result<()> {
    let body = match body {
        Some(body) => body,
        None => {
            let mut body = String::new();
            crate::editor(&mut body, Some("md")).await?;
            body
        }
    };
    api.issue_create_comment(
        repo.owner(),
        repo.name(),
        issue,
        forgejo_api::structs::CreateIssueCommentOption {
            body,
            updated_at: None,
        },
    )
    .await?;
    Ok(())
}

pub async fn edit_title(
    repo: &RepoName,
    api: &Forgejo,
    issue: u64,
    new_title: Option<String>,
) -> eyre::Result<()> {
    let new_title = match new_title {
        Some(s) => s,
        None => {
            let issue_info = api
                .issue_get_issue(repo.owner(), repo.name(), issue)
                .await?;
            let mut title = issue_info
                .title
                .ok_or_else(|| eyre::eyre!("issue does not have title"))?;
            crate::editor(&mut title, Some("md")).await?;
            title
        }
    };
    let new_title = new_title.trim();
    if new_title.is_empty() {
        eyre::bail!("title cannot be empty");
    }
    if new_title.contains('\n') {
        eyre::bail!("title cannot contain newlines");
    }
    api.issue_edit_issue(
        repo.owner(),
        repo.name(),
        issue,
        forgejo_api::structs::EditIssueOption {
            title: Some(new_title.to_owned()),
            assignee: None,
            assignees: None,
            body: None,
            due_date: None,
            milestone: None,
            r#ref: None,
            state: None,
            unset_due_date: None,
            updated_at: None,
        },
    )
    .await?;
    Ok(())
}

pub async fn edit_body(
    repo: &RepoName,
    api: &Forgejo,
    issue: u64,
    new_body: Option<String>,
) -> eyre::Result<()> {
    let new_body = match new_body {
        Some(s) => s,
        None => {
            let issue_info = api
                .issue_get_issue(repo.owner(), repo.name(), issue)
                .await?;
            let mut body = issue_info
                .body
                .ok_or_else(|| eyre::eyre!("issue does not have body"))?;
            crate::editor(&mut body, Some("md")).await?;
            body
        }
    };
    api.issue_edit_issue(
        repo.owner(),
        repo.name(),
        issue,
        forgejo_api::structs::EditIssueOption {
            body: Some(new_body),
            assignee: None,
            assignees: None,
            due_date: None,
            milestone: None,
            r#ref: None,
            state: None,
            title: None,
            unset_due_date: None,
            updated_at: None,
        },
    )
    .await?;
    Ok(())
}

pub async fn edit_comment(
    repo: &RepoName,
    api: &Forgejo,
    issue: u64,
    idx: usize,
    new_body: Option<String>,
) -> eyre::Result<()> {
    let (_, comments) = api
        .issue_get_comments(
            repo.owner(),
            repo.name(),
            issue,
            IssueGetCommentsQuery {
                since: None,
                before: None,
            },
        )
        .await?;
    let comment = comments
        .get(idx)
        .ok_or_else(|| eyre!("comment not found"))?;
    let new_body = match new_body {
        Some(s) => s,
        None => {
            let mut body = comment
                .body
                .clone()
                .ok_or_else(|| eyre::eyre!("issue does not have body"))?;
            crate::editor(&mut body, Some("md")).await?;
            body
        }
    };
    let id = comment
        .id
        .ok_or_else(|| eyre::eyre!("comment does not have id"))? as u64;
    api.issue_edit_comment(
        repo.owner(),
        repo.name(),
        id,
        forgejo_api::structs::EditIssueCommentOption {
            body: new_body,
            updated_at: None,
        },
    )
    .await?;
    Ok(())
}

pub async fn close_issue(
    repo: &RepoName,
    api: &Forgejo,
    issue: u64,
    message: Option<Option<String>>,
) -> eyre::Result<()> {
    if let Some(message) = message {
        let body = match message {
            Some(m) => m,
            None => {
                let mut s = String::new();
                crate::editor(&mut s, Some("md")).await?;
                s
            }
        };

        let opt = CreateIssueCommentOption {
            body,
            updated_at: None,
        };
        api.issue_create_comment(repo.owner(), repo.name(), issue, opt)
            .await?;
    }

    let edit = EditIssueOption {
        state: Some("closed".into()),
        assignee: None,
        assignees: None,
        body: None,
        due_date: None,
        milestone: None,
        r#ref: None,
        title: None,
        unset_due_date: None,
        updated_at: None,
    };
    let issue_data = api
        .issue_edit_issue(repo.owner(), repo.name(), issue, edit)
        .await?;

    let issue_title = issue_data
        .title
        .as_deref()
        .ok_or_eyre("issue does not have title")?;

    println!("Closed issue {issue}: \"{issue_title}\"");

    Ok(())
}
