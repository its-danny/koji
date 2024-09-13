use std::{fs::File, io::Write};

use anyhow::Result;
#[cfg(feature = "cocogitto")]
use cocogitto::CocoGitto;
use git2::Repository;

pub struct CommitParts {
    pub commit_type: String,
    pub scope: Option<String>,
    pub summary: String,
    pub body: Option<String>,
    // pub footer: Option<String>,
    pub breaking: bool,
}

pub fn write_commit_msg(repo: Repository, parts: CommitParts) -> Result<()> {
    // TODO
    #[cfg(feature = "cocogitto")]
    let message = CocoGitto::get_conventional_message(
        &parts.commit_type,
        parts.scope,
        parts.summary,
        parts.body,
        None,
        parts.breaking,
    )?;
    #[cfg(not(feature = "cocogitto"))]
    let message = get_message(parts)?;

    let commit_editmsg = repo.path().join("COMMIT_EDITMSG");
    let mut file = File::create(commit_editmsg)?;

    file.write_all(message.as_bytes())?;

    Ok(())
}

pub fn commit(parts: CommitParts, sign: bool) -> Result<()> {
    #[cfg(feature = "cocogitto")]
    CocoGitto::get()?.conventional_commit(
        &parts.commit_type,
        parts.scope,
        parts.summary,
        parts.body,
        None,
        parts.breaking,
        sign,
    )?;
    #[cfg(not(feature = "cocogitto"))]
    conventional_commit(parts)?;

    Ok(())
}

#[cfg(not(feature = "cocogitto"))]
fn _conventional_commit(parts: CommitParts) -> Result<()> {
    let message = get_message(parts)?;

    Ok(())
}

#[cfg(not(feature = "cocogitto"))]
fn get_message(parts: CommitParts) -> Result<String> {
    Ok("".to_owned())
}
