use clap::Subcommand;

#[derive(Subcommand, Clone, Debug)]
pub enum AuthCommand {
    Login,
    Logout {
        host: String,
    },
    AddKey {
        /// The domain name of the forgejo instance.
        host: String,
        /// The user that the key is associated with
        user: String,
        /// The key to add. If not present, the key will be read in from stdin.
        key: Option<String>,
    },
    List,
}

impl AuthCommand {
    pub async fn run(self, keys: &mut crate::KeyInfo) -> eyre::Result<()> {
        match self {
            AuthCommand::Login => {
                todo!();
                // let user = readline("username: ").await?;
                // let pass = readline("password: ").await?;
            }
            AuthCommand::Logout { host } => {
                let info_opt = keys.hosts.remove(&host);
                if let Some(info) = info_opt {
                    eprintln!("signed out of {}@{}", &info.username(), host);
                } else {
                    eprintln!("already not signed in to {host}");
                }
            }
            AuthCommand::AddKey { host, user, key } => {
                let key = match key {
                    Some(key) => key,
                    None => crate::readline("new key: ").await?,
                };
                if keys.hosts.get(&user).is_none() {
                    keys.hosts.insert(host, crate::keys::LoginInfo::new(user, key));
                } else {
                    println!("key for {} already exists", host);
                }
            }
            AuthCommand::List => {
                if keys.hosts.is_empty() {
                    println!("No logins.");
                }
                for (host_url, login_info) in &keys.hosts {
                    println!("{}@{}", login_info.username(), host_url);
                }
            }
        }
        Ok(())
    }
}

