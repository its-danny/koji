use std::{fs::File, io::Write};

use anyhow::Result;
#[cfg(feature = "cocogitto")]
use cocogitto::CocoGitto;
use git2::Repository;

/// Output a commit message to `.git/COMMIT_EDITMSG`
pub fn write_commit_msg(
    repo: Repository,
    commit_type: String,
    scope: Option<String>,
    summary: String,
    body: Option<String>,
    is_breaking_change: bool,
) -> Result<()> {
    let message = CocoGitto::get_conventional_message(
        &commit_type,
        scope,
        summary,
        body,
        None,
        is_breaking_change,
    )?;

    let commit_editmsg = repo.path().join("COMMIT_EDITMSG");
    let mut file = File::create(commit_editmsg)?;

    file.write_all(message.as_bytes())?;

    Ok(())
}

/// Create a commit
pub fn commit(
    commit_type: String,
    scope: Option<String>,
    summary: String,
    body: Option<String>,
    is_breaking_change: bool,
    sign: bool,
) -> Result<()> {
    let cocogitto = CocoGitto::get()?;

    cocogitto.conventional_commit(
        &commit_type,
        scope,
        summary,
        body,
        None,
        is_breaking_change,
        sign,
    )?;

    Ok(())
}
