use std::{collections::HashMap, env};

use git2::Repository;

fn main() {
    let mut environment = HashMap::new();

    // push default values
    environment.insert("GIT_COMMIT_HASH", "local env".to_string());
    environment.insert("GIT_REMOTE", "local env".to_string());
    
    if env::var("GH_ACTIONS").is_ok() {
        // only do this in gh actions, this tooks lots of time on local development env
        // This lags `cargo check` and rust-analyzer

        if let Ok(repo) = Repository::open(".") {
            if let Ok(id) = repo.refname_to_id("HEAD") {
                // get latest git commit
                if let Ok(commit) = repo.find_commit(id) {
                    let commit_id = commit.id().to_string();
                    environment.insert("GIT_COMMIT_HASH", commit_id);
                }

                // get git remote
                if let Ok(remote) = repo.find_remote("origin") {
                    if let Some(url) = remote.url() {
                        environment.insert("GIT_REMOTE", url.to_string());
                    }
                };
            }
        }
    }

    // update environment
    for (k, v) in environment {
        set_env(k, &v);
    }
}

fn set_env(key: &str, value: &str) {
    println!("cargo:rustc-env={key}={value}");
}
