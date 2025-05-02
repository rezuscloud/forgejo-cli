use std::fmt::Write;

fn main() {
    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").expect("always present"));
    println!(
        "cargo:rustc-env=BUILD_TARGET={}",
        std::env::var("TARGET").unwrap()
    );

    println!("cargo::rerun-if-env-changed=BUILTIN_CLIENT_IDS");
    let mut client_info_branches = String::new();
    if let Ok(oauth_supported) = std::env::var("BUILTIN_CLIENT_IDS") {
        for info in oauth_supported.split(",") {
            let Some((domain, id)) = info.split_once(" ") else {
                println!("cargo::warning=BUILTIN_CLIENT_IDS is set improperly");
                continue;
            };
            writeln!(&mut client_info_branches, "\"{domain}\" => Some(\"{id}\"),")
                .expect("writing to string can't fail");
        }
    }
    let oauth_match = format!(
        "match host {{
        {client_info_branches}
        _ => None,
    }}"
    );
    let oauth_src_file_path = out_dir.join("oauth_client_info.rs");
    std::fs::write(oauth_src_file_path, oauth_match)
        .expect("Failed to write oauth client info file");
}
