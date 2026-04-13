# No need to translate this.
test-hello = Hello

# No need to translate this.
test-fallback-only-english = This message is only in english

# No need to translate this.
test-placeables = Hello, {$name}! You're {$name}.

# No need to translate this.
test-switch = { $n -> 
    [one] A thing
   *[other] { $n } things
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

msg-auth-add_key-already_exists = key for {$host} already exists

msg-auth-list-none = No logins.
