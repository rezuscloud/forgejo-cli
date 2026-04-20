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

msg-org-team-delete-confirmation = Are you sure you want to delete {STYLE("bold")}{$org}/{$name}{STYLE("reset")}?
    .option-yes = Yes
    .option-yes = yes
    .option-yes = Y
    .option-yes = y
    .option-no = No
    .option-no = no
    .option-no = N
    .option-no = n

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

