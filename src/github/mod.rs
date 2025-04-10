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
    pub disk_usage: u64,
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
    username: String,
    token: String,
}

#[must_use]
pub fn auth(username: String, token: String) -> Auth {
    Auth { username, token }
}

impl From<RawRepository> for Repository {
    fn from(raw: RawRepository) -> Self {
        Self {
            name: raw.name,
            owner: raw.owner.login,
            disk_usage: raw.disk_usage,
        }
    }
}

pub fn list_repos(auth: &Auth) -> Result<Vec<Repository>, Error> {
    let args = [
        "repo",
        "list",
        &auth.username,
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
    let _output = run_command(&auth.token, "gh", args)?;
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
