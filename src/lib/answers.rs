use anyhow::{Context, Result};
use indexmap::IndexMap;
use requestty::{Answer, Answers};

use crate::{
    config::{CommitType, Config},
    emoji::ReplaceEmoji,
    questions::{
        Q_BODY, Q_COMMIT_TYPE, Q_HAS_OPEN_ISSUE, Q_ISSUE_REFERENCE, Q_IS_BREAKING_CHANGE, Q_SCOPE,
        Q_SUMMARY,
    },
};

/// Parse the commit type out of the menu choice
/// e.g. `feat: A new feature` -> `feat`
fn get_commit_type(answer: Option<&Answer>) -> Result<&str> {
    answer
        .context("could not get commit type")?
        .as_list_item()
        .context("could not convert commit type to list item")?
        .text
        .split(':')
        .next()
        .context("could not extract commit type")
}

/// Get the scope, returning `None` if it's an empty string
fn get_scope(answer: Option<&Answer>) -> Result<Option<String>> {
    answer
        .context("could not get scope")?
        .as_string()
        .context("could not convert scope to string")
        .map(|scope| {
            if scope.is_empty() {
                None
            } else {
                Some(scope.replace_emoji_shortcodes())
            }
        })
}

/// Get the summary, prepending a relevant emoji if enabled
fn get_summary(
    answer: Option<&Answer>,
    use_emoji: bool,
    commit_type: &str,
    commit_types: &IndexMap<String, CommitType>,
) -> Result<String> {
    answer
        .context("could not get summary")?
        .as_string()
        .context("could not convert summary to string")
        .map(|summary| {
            let commit_type = commit_types.get(commit_type).unwrap();
            let use_emoji = use_emoji && commit_type.emoji.is_some();
            let summary = summary.replace_emoji_shortcodes();

            if use_emoji {
                if let Some(emoji) = &commit_type.emoji {
                    format!("{emoji} {summary}")
                } else {
                    summary
                }
            } else {
                summary
            }
        })
}

/// Get the body, returning `None` if it's an empty string
fn get_body(answer: Option<&Answer>) -> Result<Option<String>> {
    answer
        .context("could not get body")?
        .as_string()
        .context("could not convert body to string")
        .map(|body| {
            if body.is_empty() {
                None
            } else {
                Some(body.replace_emoji_shortcodes())
            }
        })
}

/// Return whether or not there's a breaking change
fn get_is_breaking_change(answer: Option<&Answer>) -> Result<bool> {
    Ok(answer
        .unwrap_or(&Answer::Bool(false))
        .as_bool()
        .unwrap_or(false))
}

/// Return whether or not there's an open issue
fn get_has_open_issue(answer: Option<&Answer>) -> Result<bool> {
    Ok(answer
        .unwrap_or(&Answer::Bool(false))
        .as_bool()
        .unwrap_or(false))
}

/// Get the issue reference, returning `None` if there isn't
/// an open issue
fn get_issue_reference(answer: Option<&Answer>, has_open_issue: bool) -> Result<Option<String>> {
    if has_open_issue {
        answer
            .context("could not get issue reference")?
            .as_string()
            .context("could not convert issue reference to string")
            .map(|i| if i.is_empty() { None } else { Some(i.into()) })
    } else {
        Ok(None)
    }
}

/// If there is a referenced issue, we want to return a new string
/// appending it to the body. If not, just give back the body
fn get_amended_body(body: &Option<String>, issue_reference: &Option<String>) -> Option<String> {
    match (body, issue_reference) {
        (Some(body), Some(issue_reference)) => Some(format!("{body}\n\n{issue_reference}")),
        (Some(body), None) => Some(body.into()),
        (None, Some(issue_reference)) => Some(issue_reference.to_owned()),
        (None, None) => None,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ExtractedAnswers {
    pub body: Option<String>,
    pub commit_type: String,
    pub is_breaking_change: bool,
    pub scope: Option<String>,
    pub summary: String,
}

/// Extract the prompt answers into an `ExtractedAnswers`,
/// making it usable for creating a commit
pub fn get_extracted_answers(answers: &Answers, config: &Config) -> Result<ExtractedAnswers> {
    let commit_type = get_commit_type(answers.get(Q_COMMIT_TYPE))?.to_string();
    let scope = get_scope(answers.get(Q_SCOPE))?;
    let summary = get_summary(
        answers.get(Q_SUMMARY),
        config.emoji,
        &commit_type,
        &config.commit_types,
    )?;
    let body = get_body(answers.get(Q_BODY))?;
    let is_breaking_change = get_is_breaking_change(answers.get(Q_IS_BREAKING_CHANGE))?;
    let has_open_issue = get_has_open_issue(answers.get(Q_HAS_OPEN_ISSUE))?;
    let issue_reference = get_issue_reference(answers.get(Q_ISSUE_REFERENCE), has_open_issue)?;
    let body = get_amended_body(&body, &issue_reference);

    Ok(ExtractedAnswers {
        commit_type,
        scope,
        summary,
        body,
        is_breaking_change,
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    // TODO optional, figure out how to run tests for both with and without cocogitto feature
    
    use requestty::ListItem;

    use crate::config::Config;

    use super::*;

    #[test]
    fn test_get_commit_type() {
        let answer = Some(Answer::ListItem(ListItem {
            index: 0,
            text: "docs: Documentation only changes".into(),
        }));

        assert_eq!(get_commit_type(answer.as_ref()).unwrap(), "docs")
    }

    #[test]
    fn test_get_scope() {
        let answer = Some(Answer::String("readme".into()));
        assert_eq!(get_scope(answer.as_ref()).unwrap(), Some("readme".into()));
    }

    #[test]
    fn test_get_scope_with_shortcode() {
        let answer = Some(Answer::String(":books: readme".into()));

        assert_eq!(
            get_scope(answer.as_ref()).unwrap(),
            Some("📚 readme".into())
        );
    }

    #[test]
    fn test_get_scope_empty() {
        let answer = Some(Answer::String("".into()));
        assert_eq!(get_scope(answer.as_ref()).unwrap(), None);
    }

    #[test]
    fn test_get_summary() {
        let config = Config::new(None).unwrap();
        let commit_types = config.commit_types;

        let answer = Some(Answer::String("needed more badges".into()));

        assert_eq!(
            get_summary(answer.as_ref(), false, "docs", &commit_types).unwrap(),
            "needed more badges"
        );
    }

    #[test]
    fn test_get_summary_with_emoji() {
        let config = Config::new(None).unwrap();
        let commit_types = config.commit_types;

        let answer = Some(Answer::String("needed more badges".into()));

        assert_eq!(
            get_summary(answer.as_ref(), true, "docs", &commit_types).unwrap(),
            "📚 needed more badges"
        );
    }

    #[test]
    fn test_get_summary_with_shortcode() {
        let config = Config::new(None).unwrap();
        let commit_types = config.commit_types;

        let answer = Some(Answer::String("needed more badges :badger:".into()));

        assert_eq!(
            get_summary(answer.as_ref(), false, "docs", &commit_types).unwrap(),
            "needed more badges 🦡"
        );
    }

    #[test]
    fn test_get_body() {
        let answer = Some(Answer::String("i _really_ like badges".into()));

        assert_eq!(
            get_body(answer.as_ref()).unwrap(),
            Some("i _really_ like badges".into())
        );
    }

    #[test]
    fn test_get_body_with_shortcode() {
        let answer = Some(Answer::String("i _really_ like badges :badger:".into()));

        assert_eq!(
            get_body(answer.as_ref()).unwrap(),
            Some("i _really_ like badges 🦡".into())
        );
    }

    #[test]
    fn test_get_body_empty() {
        let answer = Some(Answer::String("".into()));
        assert_eq!(get_body(answer.as_ref()).unwrap(), None);
    }

    #[test]
    fn test_get_is_breaking_change() {
        let answer = Some(Answer::Bool(true));
        assert!(get_is_breaking_change(answer.as_ref()).unwrap(), "{}", true);
    }

    #[test]
    fn test_get_has_open_issue() {
        let answer = Some(Answer::Bool(true));
        assert!(get_has_open_issue(answer.as_ref()).unwrap(), "{}", true);
    }

    #[test]
    fn test_get_issue_reference() {
        let answer = Some(Answer::String("closes #1".into()));

        assert_eq!(
            get_issue_reference(answer.as_ref(), true).unwrap(),
            Some("closes #1".into())
        );
    }

    #[test]
    fn test_get_issue_reference_empty() {
        let answer = Some(Answer::String("".into()));
        assert_eq!(get_issue_reference(answer.as_ref(), true).unwrap(), None);
    }

    #[test]
    fn test_get_amended_body() {
        let body = Some("i _really_ like badges".to_string());
        let issue_reference = Some("closes #1".to_string());

        assert_eq!(
            get_amended_body(&body, &issue_reference),
            Some("i _really_ like badges\n\ncloses #1".into())
        );

        assert_eq!(
            get_amended_body(&body, &None),
            Some("i _really_ like badges".into())
        );

        assert_eq!(
            get_amended_body(&None, &issue_reference),
            Some("closes #1".into())
        );

        assert_eq!(get_amended_body(&None, &None), None);
    }

    #[test]
    fn test_get_extracted_answers() {
        let answers = Answers::from(HashMap::from([
            (
                Q_COMMIT_TYPE.into(),
                Answer::ListItem(ListItem {
                    index: 0,
                    text: "feat: A new feature".into(),
                }),
            ),
            (Q_SCOPE.into(), Answer::String("space".into())),
            (Q_SUMMARY.into(), Answer::String("add more space".into())),
            (
                Q_BODY.into(),
                Answer::String("just never enough space!".into()),
            ),
            (Q_IS_BREAKING_CHANGE.into(), Answer::Bool(false)),
            (Q_HAS_OPEN_ISSUE.into(), Answer::Bool(true)),
            (
                Q_ISSUE_REFERENCE.into(),
                Answer::String("closes #554".into()),
            ),
        ]));

        let config = Config::new(None).unwrap();
        let extracted_answers = get_extracted_answers(&answers, &config).unwrap();

        assert_eq!(
            extracted_answers,
            ExtractedAnswers {
                commit_type: "feat".into(),
                scope: Some("space".into()),
                summary: "add more space".into(),
                body: Some("just never enough space!\n\ncloses #554".into()),
                is_breaking_change: false,
            }
        );
    }
}
