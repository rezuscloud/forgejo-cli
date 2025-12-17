use clap::{self, Args};
use eyre::{Context, OptionExt};

use crate::{repo::RepoInfo, KeyInfo};

#[derive(Args, Clone, Debug)]
pub struct WhoAmICommand {
    #[clap(long, short)]
    remote: Option<String>,
}

impl WhoAmICommand {
    pub async fn run(self, keys: &mut KeyInfo, host_name: Option<&str>) -> eyre::Result<()> {
        let url = RepoInfo::get_current(host_name, None, self.remote.as_deref(), &keys)
            .wrap_err("could not find host, try specifying with --host")?
            .host_url()
            .clone();
        let name = keys.get_login(&url).ok_or_eyre("not logged in")?.username();
        let host = crate::host_name(&url);
        if url.path() == "/" || url.path().is_empty() {
            println!("currently signed in to {name}@{host}");
        } else {
            println!("currently signed in to {name}@{host}{}", url.path());
        };
        Ok(())
    }
}
