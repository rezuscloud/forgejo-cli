-dash =
    { IS_MINIMAL() ->
        [yes] -
       *[no] —
    }

msg-whoami = currently signed into {$name}@{$host}

msg-auth-login-oauth_unsupported = 
  Your installation of fj doesn't support `login` for {$host_domain}
  
  Please visit {$applications_url}
  to create a token, and use it to log in with `fj auth add-key`
msg-auth-login-canceled = Login canceled
msg-auth-login-browser_success = Authenticated! Close this tab and head back to your terminal.
msg-auth-login-browser_failure = Failed to authenticate.

msg-auth_logout-success = signed out of {$username}@{$host}
msg-auth_logout-already_signed_out = already not signed in to {$host}

msg-auth-use_ssh-not-logged-in = not logged in to {$host}
msg-auth-use_ssh-enabled = now will use SSH for {$host} by default
msg-auth-use_ssh-disabled = will no longer use SSH for {$host} by default
msg-auth-use_ssh-already_enabled = already using SSH for {$host} by default
msg-auth-use_ssh-already_disabled = already not using SSH for {$host} by default

msg-auth-add_key-prompt = new key: 
msg-auth-add_key-already_exists = key for {$host} already exists

msg-auth-list-none = No logins.

msg-actions-variable-create-already_exists = variable already exists, pass --force to replace it.
msg-actions-variable-create-already_exists_forced = variable already exists, updating.

msg-actions-variable-delete-success = Variable {$name} deleted.

msg-actions-dispatch-success = Dispatched workflow {$name} in {$ref} with {$n_inputs ->
        [one] 1 input
       *[other] {$n_inputs} inputs
    }.

msg-org-list-no_results = No results.
msg-org-list-page_number = Page {$page} of {$total}

msg-org-view-org_name = { OPT($full_name) ->
       *[none] {STYLE("bold", "bright-cyan")}{$name}{STYLE("reset")}
        [some] {STYLE("bold", "bright-cyan")}{$full_name}{STYLE("reset")} {STYLE("light-gray")}({$name}){STYLE("reset")}
    }
msg-org-view-visibility = { $visibility ->
        [public] Public
        [limited] Limited
       *[private] Private
    }
msg-org-view-member_count = {$member_count ->
        [one] {STYLE("bold")} 1 {STYLE("reset")} member
       *[other] {STYLE("bold")}{$member_count}{STYLE("reset")} members
    }
msg-org-view-team_count = {$team_count ->
        [one] {STYLE("bold")} 1 {STYLE("reset")} team
       *[other] {STYLE("bold")}{$team_count}{STYLE("reset")} teams
    }

msg-org-create-invalid_character = 
    Organization names can only have alphanumeric characters, dashes, underscores, or periods.
      If you want a name with other characters, try setting the --full-name flag
msg-org-create-invalid_starting_character = 
    Organization names can only start with alphanumeric characters.
      If you want a name that starts with other characters, try setting the --full-name flag
msg-org-create-invalid_ending_character =
    Organization names can only end with alphanumeric characters.
      If you want a name that ends with other characters, try setting the --full-name flag
msg-org-create-invalid_consecutive_characters =
    Organization names can't have consecutive non-alphanumeric characters.
      If you want that in the name, try setting the --full-name flag
msg-org-create-success = created new {$visibility ->
        [public] public
        [limited] limited
       *[private] private
    } org { OPT($full_name) ->
       *[none] {STYLE("bold", "bright-cyan")}{$name}{STYLE("reset")}
        [some] {STYLE("bold", "bright-cyan")}{$full_name}{STYLE("reset")} {STYLE("light-gray")}({$name}){STYLE("reset")}
    }

msg-org-members-no_results = No results.
msg-org-members-page_number = Page {$page} of {$total}
msg-org-members-entry = { OPT($full_name) ->
       *[none] {STYLE("bold", "bright-cyan")}{$username}{STYLE("reset")}
        [some] {STYLE("bold", "bright-cyan")}{$full_name}{STYLE("reset")} {STYLE("light-gray")}({$username}){STYLE("reset")}
    }

msg-org-visibility-public = You are a public member of {STYLE("bold", "bright-cyan")}{$org_name}{STYLE("reset")}
msg-org-visibility-private = You are a private member of {STYLE("bold", "bright-cyan")}{$org_name}{STYLE("reset")}
msg-org-visibility-set_public = You are now a public member of {STYLE("bold", "bright-cyan")}{$org_name}{STYLE("reset")}
msg-org-visibility-set_private = You are now a private member of {STYLE("bold", "bright-cyan")}{$org_name}{STYLE("reset")}
msg-org-visibility-not_member = You are not a member of {STYLE("bold", "bright-cyan")}{$org_name}{STYLE("reset")}

msg-org-label-add-success = Created new label {$label}

msg-org-label-edit-success = Changed label {$old_label} to {$label}

msg-org-label-remove-success = Removed label {$label}

msg-org-repo-list-no_results = No results.
msg-org-repo-list-page_number = Page {$page} of {$total}

msg-org-team-view = {STYLE("bright-blue", "bold")}{$name}{STYLE("reset")} in org {STYLE("bold")}{$org}{STYLE("reset")} {$admin ->
        [yes] {-dash} {STYLE("bright-red")}Admin{STYLE("reset")}
       *[no] {""}
    }
msg-org-team-view-read_only = Read Only:
msg-org-team-view-read_write = Read/Write:
msg-org-team-view-perms-wiki = Wikis
msg-org-team-view-perms-ext_wiki = External Wikis
msg-org-team-view-perms-issues = Issues
msg-org-team-view-perms-ext_issues = External Issues
msg-org-team-view-perms-pulls = Pull Requests
msg-org-team-view-perms-projects = Projects
msg-org-team-view-perms-actions = CI
msg-org-team-view-perms-code = Code
msg-org-team-view-perms-releases = Releases
msg-org-team-view-perms-packages = Packages

msg-org-team-create-success = created new {$admin ->
        [yes] admin
       *[no] {""}
    } team {STYLE("bright-blue", "bold")}{$name}{STYLE("reset")} in org {STYLE("bold")}{$org}{STYLE("reset")}

msg-org-team-delete-confirmation = Are you sure you want to delete {STYLE("bold")}{$org}/{$name}{STYLE("reset")}?
    .yes =
        Yes
        yes
        Y
        y
    .no =
        No
        no
        N
        n

msg-org-team-repo-list-no_results = No results.
msg-org-team-repo-list-page_number = Page {$page} of {$total}

msg-org-team-repo-add-success =
    Added {STYLE("bold")}{$org}/{$repo}{STYLE("reset")} to team {STYLE("bold", "bright_blue")}{$team}{STYLE("reset")}

msg-org-team-repo-rm-success =
    Removed {STYLE("bold")}{$org}/{$repo}{STYLE("reset")} from team {STYLE("bold", "bright_blue")}{$team}{STYLE("reset")}

msg-org-team-member-list-no_results = No results.
msg-org-team-member-list-page_number = Page {$page} of {$total}

msg-org-team-member-add-success =
    Added {STYLE("bold", "bright-cyan")}{$user}{STYLE("reset")} to team {STYLE("bold", "bright_blue")}{$team}{STYLE("reset")}

msg-org-team-member-rm-success =
    Removed {STYLE("bold", "bright-cyan")}{$user}{STYLE("reset")} from team {STYLE("bold", "bright_blue")}{$team}{STYLE("reset")}

msg-issue-create-no_templates = {$owner}/{$repo} does not have any issue templates
msg-issue-create-templates_required =
    {$owner}/{$repo} requires using a template.
    Please choose one with `--template <NAME>`.
msg-issue-create-templates_enabled =
    {$owner}/{$repo} uses issue templates.
    Please choose one with `--template <NAME>`,
    or use `--no-template` to write one from scratch".
msg-issue-create-success = created issue #{$number}: {$title}

msg-issue-view-header = 
    {STYLE("yellow")}{$title} {STYLE("dark-grey")}#{$number}{STYLE("reset")}"
    By {STYLE("white")}{$author}{STYLE("reset")} {-dash} {$state ->
        [open] {STYLE("bright-green")}Open{STYLE("reset")}
        [closed] {STYLE("bright-red")}Closed{STYLE("reset")}
       *[other] $state
    }
msg-issue-view-comment_count = { $comments ->
        [one] 1 comments
       *[other] {$comments} comments
    }

msg-issue-search-total = { $issues ->
        [one] 1 issue
       *[other] {$issues} issues
    }
msg-issue-search-entry = #{$number}: {$title} (by {$author})

msg-issue-templates-none = No issue templates or contact info.
msg-issue-templates-blank_allowed = '--no-template' is allowed
msg-issue-templates-blank_not_allowed = '--no-template' is not allowed

msg-issue-view-comments-comment_header = { OPT($full_name) ->
       *[none] {STYLE("bold", "bright-cyan")}{$username}{STYLE("reset")} said:
        [some] {STYLE("bold", "bright-cyan")}{$full_name}{STYLE("reset")} {STYLE("dark-gray")}({$username}){STYLE("reset")} said:
    }
msg-issue-view-comments-attachments = { $attachments ->
        [one] 1 attachment
       *[other] {$attachments} attachments
    }

msg-issue-edit-title-empty = title cannot be empty
msg-issue-edit-title-no_newlines = title cannot contain newlines

msg-issue-assign-success =
    assigned {$added ->
        [one] 1 user
       *[other] {$added} users
    } to {$owner}/{$repo}#{$number} {$duplicate ->
        [0] {""}
        [one] {$added ->
            [0] (user was already assigned)
           *[other] (1 user was already assigned)
        }
       *[other] {$added ->
            [0] (all users were already assigned)
           *[other] ({$duplicate} users were already assigned)
        }
    }

msg-issue-unassign-success =
    unassigned {$removed ->
        [one] 1 user
       *[other] {$removed} users
    } from {$owner}/{$repo}#{$number} {$duplicate ->
        [0] {""}
        [one] {$removed ->
            [0] (user was already not assigned)
           *[other] (1 user was already not assigned)
        }
       *[other] {$removed ->
            [0] (all users were already not assigned)
           *[other] ({$duplicate} users were already not assigned)
        }
    }

msg-issue-close-success = Closed issue #{$number}: "{$title}"

msg-pr-couldnt_guess = could not guess pull request number, please specify
msg-pr-not_found = could not find PR

msg-pr-view-header =
    {STYLE("yellow")}{$title} {STYLE("dark-grey")}#{$number}{STYLE("reset")}
    By {STYLE("white")}{$username}{STYLE("reset")} {-dash} {$state ->
        [draft] {STYLE("light-grey")}Draft{STYLE("reset")}
        [open] {STYLE("bright-green")}Open{STYLE("reset")}
        [merged] {STYLE("bright-magenta")}Merged{STYLE("reset")}
        [closed] {STYLE("bright-red")}Closed{STYLE("reset")}
       *[other] $state
    } {-dash} {STYLE("bright-green")}+{$additions} {STYLE("bright-red")}-{$deletions}{STYLE("reset")}
    {OPT($head_branch) ->
       *[none] Into `{$base_branch}`
        [some] From `{$head_branch}` into `{$base_branch}`
    }
msg-pr-view-comment_count = { $comments ->
        [one] 1 comments
       *[other] {$comments} comments
    }

msg-pr-status-merged = {STYLE("bright-magenta")}Merged{STYLE("reset")} by {$merged_by} on {DATETIME($created_at, dateStyle: "long", timeStyle: "long")}
msg-pr-status-header = {$state ->
        [draft] {STYLE("light-grey")}Draft{STYLE("reset")} {-dash} Can't merge draft PR
        [open] {STYLE("bright_green")}Open{STYLE("reset")} {-dash} {$mergeable ->
           *[yes] Can be merged
            [no] {STYLE("bright-red")}Merge conflicts{STYLE("reset")}
        }
        [closed] {STYLE("bright-red")}Closed{STYLE("reset")} {-dash} Reopen to merge
       *[other] Unknown
    }
msg-pr-status-entry = {$state ->
        [success] {STYLE("bright_green")}Success{STYLE("reset")}
        [pending] {STYLE("yellow")}Pending{STYLE("reset")}
        [warning] {STYLE("bright_yellow")}Warning{STYLE("reset")}
        [failure] {STYLE("bright_red")}Failure{STYLE("reset")}
        [error] {STYLE("bright_red")}Error{STYLE("reset")}
       *[other] Unknown
    } {-dash} {$context}

msg-pr-review-list-none = No reviews.
msg-pr-review-list-only_stale = Only stale or dismissed reviews, use --all to display them.
msg-pr-review-list-review_header = {$review_type ->
        [approved] {STYLE("bright-green")}Approved{STYLE("reset")}
        [changes-requested] {STYLE("bright-yellow")}Changes requested{STYLE("reset")}
        [comment] {STYLE("bright-yellow")}Comment{STYLE("reset")}
        [pending] {STYLE("light-grey")}Pending Review{STYLE("reset")}
       *[other] Unknown
    } by {STYLE("bold")}{$reviewer}{STYLE("reset")}
    {STYLE("dark-grey")}{$comments ->
        [one] 1 comment
       *[other] {$comments} comments
    }, made on {DATETIME($timestamp, dateStyle: "long", timeStyle: "short")}{STYLE("reset")} {$state ->
        [stale] {STYLE("bold")}(stale){STYLE("reset")}
        [dismissed] {STYLE("bold")}(dismissed){STYLE("reset")}
       *[other] {""}
    }
msg-pr-review-list-comment_position = In {STYLE("bold")}{$path}:{$position}{STYLE("reset")}:
msg-pr-review-list-comment_header = {STYLE("bold", "bright-cyan")}{$commenter}{STYLE("reset")} commented {OPT($resolver) ->
       *[none] {""}
        [some] (resolved by {$resolver})
    }:

msg-pr-create-cross_instance = cannot create pull request across instances; base is on {$base_instance}, while head is tracking {$head_instance}
msg-pr-create-success = created pull request #{$number}: {$title}
msg-pr-create-agit_success = created pull request: {$title}
msg-pr-create-agit_push_cfg_question =
    Would you like to set the needed git config
    items so that `git push` works for this pr?
msg-pr-create-agit_push_cfg_prompt = (y/N/?) 
    .yes =
        Yes
        yes
        Y
        y
    .no =
        No
        no
        N
        n
    .help =
        Help
        help
        H
        h
        ?
msg-pr-create-agit_force_push_warning =
    {STYLE("bold")}Note:{STYLE("reset")}
      `git push --force[-with-lease]` is not supported for AGit PRs.
      You can use `git push -o force=true` instead.
msg-pr-create-agit_push_cfg_help = This would set the following config options:

msg-pr-merge-commit_title_unsupported-rebase = rebase does not support commit title
msg-pr-merge-commit_title_unsupported-ff = ff-only does not support commit title
msg-pr-merge-commit_title_unsupported-manual = manually merged does not support commit title
msg-pr-merge-default_message = Reviewed-on: {$pr_url}
msg-pr-merge-success = Merged PR #{$number} \"{$title}\" into `{$base_branch}`

msg-pr-checkout-dirty = Cannot checkout PR; working directory has uncommitted changes
msg-pr-checkout-not_fork = cannot get parent repo, {$repo} is not a fork
msg-pr-checkout-success = Checked out PR #{$number}: {$title}
    {$new_branch ->
       *[yes] On new branch {$branch_name}
        [no] Updated branch to latest commit
    }

msg-pr-search-count = {$pull_requests ->
        [one] 1 pull request
       *[other] {$pull_requests} pull requests
    }
msg-pr-search-entry = #{$number}: {$title} (by {$author})

msg-pr-view-diff-volatile = changes made to the diff will not persist

msg-repo-no_host_given = cannot find repo, no host specified
msg-repo-no_info_given =
    no repo info specified

    If you're trying to operate on a repository in the current directory, try adding a remote
    referencing the forgejo instance. If you have multiple remotes, try setting one as upstream to the
    current branch. You may also specify a host explicitly using the `--host` argument.

msg-repo-fallback_host-invalid_url = warn: `FJ_FALLBACK_HOST` is not set to a valid url

msg-repo-arg_no_owner = repo name should be in the format [HOST/]OWNER/NAME

msg-repo-name_needed = couldn't get repo name, please specify

msg-repo-create-remote_exists = A remote named \"{$remote_name}\" already exists
msg-repo-create-success = created new repo at {$url}
msg-repo-create-detached_head = HEAD is not on a branch; cannot push to remote
msg-repo-create-branch_invalid_utf8 = branch name invalid utf-8

msg-repo-fork-conflicting_hosts = conflicting hosts {$host_a} and {$host_b}. please only specify one
msg-repo-fork-success = Forked {$parent_owner}/{$parent_name} into {$fork_name}

msg-repo-migrate-git_only = Migrating from a `git` service doesn't support migration items other than LFS. Please specify a different service or remove the included items
msg-repo-migrate-username_prompt = Username: 
msg-repo-migrate-password_prompt = Password: 
msg-repo-migrate-token_prompt = Token: 
msg-repo-migrate-migrating = Migrating...
msg-repo-migrate-success = Done! View online at {$url}

msg-repo-view-name = {$repo_name}
msg-repo-view-is_fork = Fork of {$parent}
msg-repo-view-is_mirror = Mirror of {$mirror_of}
msg-repo-view-primary_language = Primary language is {$language}
msg-repo-view-stars = {$stars ->
        [one] 1 star
       *[other] {$stars} stars
    }
msg-repo-view-watching = {$watching} watching
msg-repo-view-forks = {$forks ->
        [one] 1 fork
       *[other] {$forks} forks
    }
msg-repo-view-issues = {$issues ->
        [one] 1 issue
       *[other] {$issues} issues
    }
msg-repo-view-prs = {$pull_requests ->
        [one] 1 PR
       *[other] {$pull_requests} PRs
    }
msg-repo-view-releases = {$releases ->
        [one] 1 release
       *[other] {$releases} releases
    }
msg-repo-view-external_tracker = Issue tracker is at {$url}
msg-repo-view-url = View online at {$url}

msg-repo-readme-none = Repo does not have a README

msg-repo-clone-preparing = {"   "}Preparing...
msg-repo-clone-downloading = {" "}Downloading... {NUMBER($percent, maximumFractionDigits: 2)}% ({NUMBER($size, maximumFractionDigits: 2)}{$units})
msg-repo-clone-resolving = {"   "}Resolving... {NUMBER($percent, maximumFractionDigits: 2)}%
msg-repo-clone-finishing_up = Finishing up...
msg-repo-clone-success = Cloned {$repo} into {$path}

msg-repo-star-success = Starred {$owner}/{$repo}!

msg-repo-unstar-success = Removed star from {$owner}/{$repo}!

msg-repo-delete-confirmation_prompt = Are you sure you want to delete {$owner}/{$name}? (y/N) 
    .yes =
        Yes
        yes
        Y
        y
    .no =
        No
        no
        N
        n
msg-repo-delete-success = Deleted {$owner}/{$repo}
msg-repo-delete-cancelled = Did not delete

msg-repo-label-view-archived = (archived)
msg-repo-label-view-no_description = (no description)

msg-repo-label-create-success = Successfully created label {$label}

msg-repo-label-delete-success = Successfully deleted label {$label}

msg-repo-label-edit-success = Edited label: {$label}

msg-user-search-page_zero = There is no page 0
msg-user-search-fail = Search failed
msg-user-search-none = No users matched that query
msg-user-search-page_too_high = {$total_pages ->
        [one] There is only 1 page
       *[other] There are only {$total_pages} pages
    }
msg-user-search-footer =
    Showing {STYLE("bold")}{$first_index}{-dash}{$last_index}{STYLE("reset")} of {STYLE("bold")}{$total_results}{STYLE("reset")} results ({$page}/{$total_pages})
    {$more ->
        [yes] View more with the --page flag
       *[no] {""}
    }

msg-user-view-header = {STYLE("bright-cyan", "bold")}{$username}{STYLE("reset")} {OPT($pronouns) ->
       *[none] {""}
        [some] {STYLE("light-grey")} {-dash} {STYLE("bold")}{$pronouns}{STYLE("reset")}
    }
    {$followers ->
        [one] {STYLE("bold")}1{STYLE("reset")} follower
       *[other] {STYLE("bold")}{$followers}{STYLE("reset")} followers
    } {-dash} {STYLE("bold")}{$following}{STYLE("reset")} following
    {OPT($website) ->
       *[none] {OPT($email) ->
           *[none] {""}
            [some] {STYLE("bold")}{$email}{STYLE("reset")}
        }
        [some] {OPT($email) ->
           *[none] {STYLE("bold")}{$website}{STYLE("reset")}
            [some] {STYLE("bold")}{$website}{STYLE("reset")} {-dash} {STYLE("bold")}{$email}{STYLE("reset")}
        }
    }
msg-user-view-joined_on = Joined on {STYLE("bold")}{DATETIME($joined, dateStyle: "medium")}{STYLE("reset")}

msg-user-follow-success = Followed {$username}

msg-user-unfollow-success = Unfollowed {$username}

msg-user-following-none-other = {$user} isn't following anyone
msg-user-following-none-self = You aren't following anyone
msg-user-following-other = {$user} is following:
msg-user-following-self = You are following:

msg-user-followers-none-other = {$user} has no followers
msg-user-followers-none-self = You have no followers :(
msg-user-followers-other = {$user} is followed by:
msg-user-followers-self = You are followed by:

msg-user-block-success = Blocked {$user}

msg-user-unblock-success = Unblocked {$user}

msg-user-repos-none-starred-other = {$name} has not starred any repos
msg-user-repos-none-starred-self = You have not starred any repos
msg-user-repos-none-other = {$name} does not own any repos
msg-user-repos-none-self = You do not own any repos
msg-user-repos-list_footer =
    Showing {STYLE("bold")}{$first_index}{-dash}{$last_index}{STYLE("reset")} of {STYLE("bold")}{$total_results}{STYLE("reset")} results ({$page}/{$total_pages})
    {$more ->
        [yes] View more with the --page flag
       *[no] {""}
    }

msg-user-orgs-none-other = {$user} is not a member of any organizations
msg-user-orgs-none-self = You are not a member of any organizations
msg-user-orgs-count = {$organizations ->
        [one] 1 organization
       *[other] {$organizations} organizations
    }

msg-activity-created_fork = {STYLE("bold")}{$actor}{STYLE("reset")} forked repository {STYLE("bold", "yellow")}{$parent_repo_name}{STYLE("reset")} to {STYLE("bold", "yellow")}{$repo_name}{STYLE("reset")}
msg-activity-created_mirror = {STYLE("bold")}{$actor}{STYLE("reset")} created mirror {STYLE("bold", "yellow")}{$repo_name}{STYLE("reset")}
msg-activity-created_repo = {STYLE("bold")}{$actor}{STYLE("reset")} created repository {STYLE("bold", "yellow")}{$repo_name}{STYLE("reset")}
msg-activity-renamed_repo = {STYLE("bold")}{$actor}{STYLE("reset")} renamed repository from {STYLE("bold", "yellow")}\"{$old_name}\"{STYLE("reset")} to {STYLE("bold", "yellow")}{$new_name}{STYLE("reset")}
msg-activity-starred_repo = {STYLE("bold")}{$actor}{STYLE("reset")} starred repository {STYLE("bold", "yellow")}{$repo_name}{STYLE("reset")}
msg-activity-watched_repo = {STYLE("bold")}{$actor}{STYLE("reset")} watched repository {STYLE("bold", "yellow")}{$repo_name}{STYLE("reset")}
msg-activity-pushed_commit = {STYLE("bold")}{$actor}{STYLE("reset")} pushed to {STYLE("bold", "bright-cyan")}{$branch}{STYLE("reset")} on {STYLE("bold", "yellow")}{$repo_name}{STYLE("reset")}
msg-activity-created_issue = {STYLE("bold")}{$actor}{STYLE("reset")} opened issue {STYLE("bold", "yellow")}{$repo_name}#{$number}{STYLE("reset")}
msg-activity-created_pr = {STYLE("bold")}{$actor}{STYLE("reset")} created pull request {STYLE("bold", "yellow")}{$repo_name}#{$number}{STYLE("reset")}
msg-activity-transferred_repo = {STYLE("bold")}{$actor}{STYLE("reset")} transferred repository {STYLE("bold", "yellow")}\"{$old_name}\"{STYLE("reset")} to {STYLE("bold", "yellow")}{$new_name}{STYLE("reset")}
msg-activity-pushed_tag = {STYLE("bold")}{$actor}{STYLE("reset")} pushed tag {STYLE("bold", "bright_cyan")}{$tag_name}{STYLE("reset")} to {STYLE("bold", "yellow")}{$repo_name}{STYLE("reset")}
msg-activity-commented_issue = {STYLE("bold")}{$actor}{STYLE("reset")} commented on issue {STYLE("bold", "yellow")}{$repo_name}#{$number}{STYLE("reset")}
msg-activity-merged_pr = {STYLE("bold")}{$actor}{STYLE("reset")} merged pull request {STYLE("bold", "yellow")}{$repo_name}#{$number}{STYLE("reset")}
msg-activity-closed_issue = {STYLE("bold")}{$actor}{STYLE("reset")} closed issue {STYLE("bold", "yellow")}{$repo_name}#{$number}{STYLE("reset")}
msg-activity-reopened_issue = {STYLE("bold")}{$actor}{STYLE("reset")} reopened issue {STYLE("bold", "yellow")}{$repo_name}#{$number}{STYLE("reset")}
msg-activity-closed_pr = {STYLE("bold")}{$actor}{STYLE("reset")} closed pr {STYLE("bold", "yellow")}{$repo_name}#{$number}{STYLE("reset")}
msg-activity-reopened_pr = {STYLE("bold")}{$actor}{STYLE("reset")} reopened pr {STYLE("bold", "yellow")}{$repo_name}#{$number}{STYLE("reset")}
msg-activity-deleted_tag = {STYLE("bold")}{$actor}{STYLE("reset")} deleted tag {STYLE("bold", "bright_cyan")}{$tag_name}{STYLE("reset")} from {STYLE("bold", "yellow")}{$repo_name}{STYLE("reset")}
msg-activity-deleted_branch = {STYLE("bold")}{$actor}{STYLE("reset")} deleted branch {STYLE("bold", "bright_cyan")}{$branch}{STYLE("reset")} from {STYLE("bold", "yellow")}{$repo_name}{STYLE("reset")}
msg-activity-approved_pr = {STYLE("bold")}{$actor}{STYLE("reset")} approved {STYLE("bold", "yellow")}{$repo_name}#{$number}{STYLE("reset")}
msg-activity-rejected_pr = {STYLE("bold")}{$actor}{STYLE("reset")} suggested changes for {STYLE("bold", "yellow")}{$repo_name}#{$number}{STYLE("reset")}
msg-activity-commented_pr = {STYLE("bold")}{$actor}{STYLE("reset")} commented on pull request {STYLE("bold", "yellow")}{$repo_name}#{$number}{STYLE("reset")}
msg-activity-created_release = {STYLE("bold")}{$actor}{STYLE("reset")} created release {STYLE("bold", "bright_cyan")}{$release_name}{STYLE("reset")} on {STYLE("bold", "yellow")}{$repo_name}{STYLE("reset")}

msg-user-edit-name-removal_hint = Use --unset to remove your name from your profile
msg-user-edit-pronouns-removal_hint = Use --unset to remove your pronouns from your profile
msg-user-edit-location-removal_hint = Use --unset to remove your location from your profile
msg-user-edit-website-removal_hint = Use --unset to remove your website from your profile

msg-user-key-list-count = total keys: {$keys}
msg-user-key-list-header = {STYLE("bold")}Key {STYLE("bright-magenta")}{$id}{STYLE("reset")}
msg-user-key-list-title = {STYLE("bold")}Title:{STYLE("reset")}       {STYLE("bright-cyan")}{$title}{STYLE("reset")}
msg-user-key-list-created_at = {STYLE("bold")}Created At:{STYLE("reset")}  {STYLE("bright-cyan")}{DATETIME($created_at, dateStyle: "short", timeStyle: "medium")}{STYLE("reset")}
msg-user-key-list-type = {STYLE("bold")}Type:{STYLE("reset")}        {STYLE("bright-cyan")}{$key_type}{STYLE("reset")}
msg-user-key-list-fingerprint = {STYLE("bold")}Fingerprint:{STYLE("reset")} {STYLE("bright-cyan")}{$fingerprint}{STYLE("reset")}

msg-user-key-delete-success = successfully deleted key with ID {$id}

msg-user-key-upload-home_not_found = Couldn't locate home directory. Please provide an explicit path for the key file.
msg-user-key-upload-keys_not_found = No keys found.
msg-user-key-upload-confirm_key_file_prompt =
        Guessed key file: {$path}
        Does this look good?
    .yes =
        Yes
        yes
        Y
        y
    .no =
        No
        no
        N
        n
msg-user-key-add-file_unconfirmed = User didn't confirm guessed key file.
msg-user-key-add-unexpected_extension = 
    '{$path}' doesn't end in '.pub'. Are you sure this isn't a private key?
     If you want to proceed anyways, add --force.
msg-user-key-add-invalid_key = 
    '{$path}' looks like a private key or invalid data!
     If you want to proceed anyways, add --force.
msg-user-key-add-no_title = Couldn't guess key title, please provide one explicitly and check your key file.
msg-user-key-upload-confirm_key_title_prompt =
        Guessed title: {STYLE("bright-cyan")}{$title}{STYLE("reset")}
        Does this look good?
    .yes =
        Yes
        yes
        Y
        y
    .no =
        No
        no
        N
        n
msg-user-key-add-title_unconfirmed = User didn't confirm guessed title.
msg-user-key-add-success = Key created successfully!

msg-user-gpg-list-count = total keys: {$keys}
msg-user-gpg-list-header = {STYLE("bold")}Key {STYLE("bright-magenta")}{$id}{STYLE("reset")}
msg-user-gpg-list-key_id = {STYLE("bold")}Key ID:{STYLE("reset")}              {STYLE("bright-cyan")}{$key_id}{STYLE("reset")}
msg-user-gpg-list-can_sign = {STYLE("bold")}Can Sign:{STYLE("reset")}            {$can_sign ->
        [yes] {STYLE("bright-green")}true{STYLE("reset")}
       *[no] {STYLE("bright-red")}false{STYLE("reset")}
    }
msg-user-gpg-list-can_encrypt_comms = {STYLE("bold")}Can Encrypt Comms:{STYLE("reset")}   {$can_encrypt_comms ->
        [yes] {STYLE("bright-green")}true{STYLE("reset")}
       *[no] {STYLE("bright-red")}false{STYLE("reset")}
    }
msg-user-gpg-list-can_encrypt_storage = {STYLE("bold")}Can Encrypt Storage:{STYLE("reset")} {$can_encrypt_storage ->
        [yes] {STYLE("bright-green")}true{STYLE("reset")}
       *[no] {STYLE("bright-red")}false{STYLE("reset")}
    }
msg-user-gpg-list-can_certify = {STYLE("bold")}Can Certify:{STYLE("reset")}         {$can_certify ->
        [yes] {STYLE("bright-green")}true{STYLE("reset")}
       *[no] {STYLE("bright-red")}false{STYLE("reset")}
    }
msg-user-gpg-list-verified = {STYLE("bold")}Verified:{STYLE("reset")}            {$verified ->
        [yes] {STYLE("bright-green")}true{STYLE("reset")}
       *[no] {STYLE("bright-red")}false{STYLE("reset")}
    }
msg-user-gpg-list-email = {STYLE("bright-cyan")}{$email}{STYLE("reset")} {$verified ->
        [yes] verified
       *[no] not verified
    }
msg-user-gpg-list-subkey = {STYLE("bold")}Subkey {STYLE("bright-magenta")}{$id}{STYLE("reset")}:

msg-user-gpg-upload-exporting = Exporting key...
msg-user-gpg-upload-export_failed = Failed to export key. {OPT($status_code) ->
       *[none] {""}
        [some] GPG status: {$status_code}
    }
msg-user-gpg-upload-success = Key successfully added!

msg-user-gpg-verify-fetching_token = Fetching verification token...
msg-user-gpg-verify-signing_token = Signing verification token with key '{$key_name}'...
msg-user-gpg-verify-signing_failed = Failed to sign verification token. {OPT($status_code) ->
       *[none] {""}
        [some] GPG status: {$status_code}
    }
msg-user-gpg-verify-key_to_verify = Verifying this key:
msg-user-gpg-verify-success = Verification successful!

msg-user-gpg-delete-confirmation_prompt =
        Deleting a GPG key will cause all commits signed by that key to become unverified! Continue?
    .yes =
        Yes
        yes
        Y
        y
    .no =
        No
        no
        N
        n
msg-user-gpg-delete-unconfirmed = User aborted process.
msg-user-gpg-delete-success = Key with ID {$id} deleted successfully.

msg-release-create-must_specify_tag = must select tag with `--tag` or `--create-tag`
msg-release-create-tag_flags_conflict =`--tag` and `--create-tag` are mutually exclusive; please pick just one 
msg-release-create-success = Created release {$name}

msg-release-list-entry = {$name} {$state ->
       *[neither] {""}
        [draft] (draft)
        [prerelease] (prerelease)
        [both] (draft, prerelease)
    }

msg-release-view-header = {$name}
    By {$author} on {DATETIME($created_at, dateStyle: "long")}

msg-release-asset-create-success = Added attachment `{$asset}` to {$release}

msg-release-asset-delete-success = Added attachment `{$asset}` to {$release}

msg-release-asset-download-success = { OPT($file) ->
       *[none] Downloaded {$asset}
        [some] Downloaded {$asset} into {$file}
    }

msg-tag-create-success = created tag {$name}

msg-tag-delete-success = created tag {$name}

msg-version-update_check-hint = Check for a new version with `fj version --check`
msg-version-update_check-current = Up to date!
msg-version-update_check-behind =
    New version available: {$new_version}
    Get it at {$url}
msg-version-update_check-ahead = You are ahead of the latest published version

msg-wiki-clone-success = Cloned {$repo}'s wiki into {$path}


