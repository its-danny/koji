use std::fs::read_to_string;

use anyhow::{Context, Result};
use clap::Parser;

use conventional_commit_parser::parse;
use git2::Repository;
use koji::answers::{get_extracted_answers, ExtractedAnswers};
use koji::commit::{commit, write_commit_msg, CommitParts};
use koji::config::{Config, ConfigArgs};
use koji::questions::create_prompt;

#[derive(Parser, Debug)]
#[command(
    about = "🦊 An interactive CLI for creating conventional commits.",
    version,
    author
)]
struct Args {
    #[arg(
        short,
        long,
        help = "Enables autocomplete for scope prompt via scanning commit history"
    )]
    autocomplete: Option<bool>,

    #[arg(short, long, help = "Enables breaking change prompt")]
    breaking_changes: Option<bool>,

    #[arg(
        short,
        long,
        help = "Path to a config file containing custom commit types"
    )]
    config: Option<String>,

    #[arg(
        short,
        long,
        help = "Prepend the commit summary with relevant emoji based on commit type"
    )]
    emoji: Option<bool>,

    #[arg(
        long,
        help = "Run as a git hook, writing the commit message to COMMIT_EDITMSG instead of committing"
    )]
    hook: bool,

    #[arg(
        short,
        long,
        help = "Enables issue prompt, which will append a reference to an issue in the commit body"
    )]
    issues: Option<bool>,

    #[arg(
        long,
        help = "Sign the commit using the user's GPG key, if one is configured"
    )]
    sign: Option<bool>,
}

fn main() -> Result<()> {
    // Get CLI args
    let Args {
        autocomplete,
        breaking_changes,
        config,
        emoji,
        hook,
        issues,
        sign,
    } = Args::parse();

    // Find repo
    let repo =
        Repository::discover(std::env::current_dir()?).context("could not find git repository")?;

    // Get existing commit message (passed in via `-m`)
    let commit_editmsg = repo.path().join("COMMIT_EDITMSG");
    let commit_message = match read_to_string(commit_editmsg) {
        Ok(contents) => contents.lines().next().unwrap_or("").to_string(),
        Err(_) => "".to_string(),
    };

    // If the existing message is already in the form of a conventional commit,
    // just go ahead and return early
    if hook && parse(&commit_message).is_ok() {
        return Ok(());
    }

    // Load config
    let config = Config::new(Some(ConfigArgs {
        autocomplete,
        breaking_changes,
        emoji,
        issues,
        path: config,
        sign,
    }))?;

    // Get answers from interactive prompt
    let answers = create_prompt(&repo, commit_message, &config)?;

    // Get data necessary for a conventional commit
    let ExtractedAnswers {
        body,
        commit_type,
        is_breaking_change,
        scope,
        summary,
    } = get_extracted_answers(&answers, &config)?;

    let commit_parts = CommitParts {
        commit_type,
        scope,
        summary,
        body,
        breaking: is_breaking_change,
    };

    // Do the thing!
    if hook {
        write_commit_msg(repo, commit_parts)?;
    } else {
        commit(commit_parts, config.sign)?;
    }

    Ok(())
}
