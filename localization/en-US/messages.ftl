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
msg-auth-use_ssh-disabled = will no longer use SSH for {host} by default
msg-auth-use_ssh-already_enabled = already using SSH for {$host} by default
msg-auth-use_ssh-already_disabled = already not using SSH for {host} by default

msg-auth-add_key-prompt = new key: 
msg-auth-add_key-already_exists = key for {$host} already exists

msg-auth-list-none = No logins.

msg-actions-variable-create-already_exists = variable already exists, pass --force to replace it.
msg-actions-variable-create-already_exists_forced = variable already exists, updating.

msg-actions-variable-delete-success = Variable {$name} deleted.

msg-actions-dispatch-success = Dispatched workflow {name} in {ref} with {n_inputs} input(s).

msg-org-list-no_results = No results.
msg-org-list-page_number = Page {$page} of {$total}

msg-org-view-org_name = { IS_NONE($full_name) ->
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
    Organization names can only have alphanumeric characters, dash, underscore, or period.
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
    } org { IS_NONE($full_name) ->
       *[none] {STYLE("bold", "bright-cyan")}{$name}{STYLE("reset")}
        [some] {STYLE("bold", "bright-cyan")}{$full_name}{STYLE("reset")} {STYLE("light-gray")}({$name}){STYLE("reset")}
    }

msg-org-members-no_results = No results.
msg-org-members-page_number = Page {$page} of {$total}
msg-org-members-entry = { IS_NONE($full_name) ->
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
    .option-yes = Yes
    .option-yes = yes
    .option-yes = Y
    .option-yes = y
    .option-no = No
    .option-no = no
    .option-no = N
    .option-no = n

msg-org-team-repo-list-no_results = No results.
msg-org-team-repo-list-page_number = Page {$page} of {$total}

msg-org-team-repo-add-success =
    Added {STYLE("bold")}{$org}/{$repo}{STYLE("reset")} to team {STYLE("bold", "bright_blue")}{$team}{STYLE("reset")}

msg-org-team-repo-rm-success =
    Removed {STYLE("bold")}{$org}/{$repo}{STYLE("reset")} from team {STYLE("bold", "bright_blue")}{$team}{STYLE("reset")}

msg-org-team-member-list-no_results = No results.
msg-org-team-member-list-page_number = Page {$page} of {$total}

msg-org-team-member-add-success =
    Added {STYLE("bold")}{$org}/{$repo}{STYLE("reset")} to team {STYLE("bold", "bright_blue")}{$team}{STYLE("reset")}

msg-org-team-member-rm-success =
    Removed {STYLE("bold", "bright-cyan")}{$org}/{$repo}{STYLE("reset")} from team {STYLE("bold", "bright_blue")}{$team}{STYLE("reset")}

msg-issue-create-no_templates = {$owner}/{$repo} does not have any issue templates
msg-issue-create-templates_required =
    {$owner}/{$repo} requires using a template.
    Please choose one with `--template <NAME>`
msg-issue-create-templates_enabled =
    {$owner}/{$repo} uses issue templates.
    Please choose one with `--template <NAME>`,
    or use `--no-template` to write one from scratch",
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

msg-issue-view-comments-comment_header = { IS_NONE($full_name) ->
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
    {IS_NONE($head_branch) ->
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
msg-pr-review-list-comment_header = {STYLE("bold", "bright-cyan")}{$commenter}{STYLE("reset")} commented {IS_NONE($resolver) ->
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
    .option-yes = Yes
    .option-yes = yes
    .option-yes = Y
    .option-yes = y
    .option-no = No
    .option-no = no
    .option-no = N
    .option-no = n
    .option-help = ?
    .option-help = h
    .option-help = H
    .option-help = help
    .option-help = Help
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

msg-repo-migrate-username_prompt = Username: 
msg-repo-migrate-password_prompt = Password: 
msg-repo-migrate-token_prompt = Token: 

msg-repo-delete-confirmation_prompt = Are you sure you want to delete {$owner}/{$name}? (y/N) 
    .option-yes = Yes
    .option-yes = yes
    .option-yes = Y
    .option-yes = y
    .option-no = No
    .option-no = no
    .option-no = N
    .option-no = n

msg-user-key-upload-confirm_key_file_prompt =
        Guessed key file: {$path}
        Does this look good?
    .option-yes = Yes
    .option-yes = yes
    .option-yes = Y
    .option-yes = y
    .option-no = No
    .option-no = no
    .option-no = N
    .option-no = n

msg-user-key-upload-confirm_key_title_prompt =
        Guessed title: {STYLE("bright-cyan")}{$title}{STYLE("reset")}
        Does this look good?
    .option-yes = Yes
    .option-yes = yes
    .option-yes = Y
    .option-yes = y
    .option-no = No
    .option-no = no
    .option-no = N
    .option-no = n

msg-user-key-delete-confirmation_prompt =
        Deleting a GPG key will cause all commits signed by that key to become unverified! Continue?
    .option-yes = Yes
    .option-yes = yes
    .option-yes = Y
    .option-yes = y
    .option-no = No
    .option-no = no
    .option-no = N
    .option-no = n

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

msg-release-asset-download-success = { IS_NONE($file) ->
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


