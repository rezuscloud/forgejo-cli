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

