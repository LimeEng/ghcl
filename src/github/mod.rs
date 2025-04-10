use serde::{Deserialize, Serialize};
use std::{
    ffi::OsStr,
    io,
    process::{Command, Output},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub name: String,
    pub owner: String,
    pub size_bytes: u64,
}

impl Repository {
    #[must_use]
    pub fn full_name(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawRepository {
    pub name: String,
    pub owner: Owner,
    pub disk_usage: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Owner {
    id: String,
    login: String,
}

#[derive(Clone, Debug)]
pub struct Auth {
    owner: String,
    token: String,
}

#[must_use]
pub fn auth(owner: String, token: String) -> Auth {
    Auth { owner, token }
}

impl From<RawRepository> for Repository {
    fn from(raw: RawRepository) -> Self {
        Self {
            name: raw.name,
            owner: raw.owner.login,
            size_bytes: raw.disk_usage * 1024,
        }
    }
}

pub fn list_repos(auth: &Auth) -> Result<Vec<Repository>, Error> {
    let args = [
        "repo",
        "list",
        &auth.owner,
        "--limit",
        "1000",
        "--json",
        "name,owner,diskUsage",
    ];
    let output = run_command(&auth.token, "gh", args)?;
    let repos: Vec<RawRepository> = serde_json::from_slice(&output.stdout)?;
    let repos: Vec<Repository> = repos.iter().map(|raw| raw.clone().into()).collect();
    Ok(repos)
}

pub fn clone_repo(auth: &Auth, repository: &Repository) -> Result<(), Error> {
    let args = [
        "repo",
        "clone",
        &repository.full_name(),
        &repository.full_name(),
    ];
    run_command(&auth.token, "gh", args)?;
    Ok(())
}

fn run_command<T, S>(token: &str, program: &str, args: T) -> io::Result<Output>
where
    T: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Command::new(program)
        .args(args)
        .env("GH_TOKEN", token)
        .output()
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Serde(serde_json::Error),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Serde(error)
    }
}
