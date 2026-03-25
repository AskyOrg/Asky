use regex::Regex;
use std::{borrow::Cow, env::VarError, fmt::Write, sync::LazyLock};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnvPlaceholderError {
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(#[from] VarError),

    #[error("Format error: {0}")]
    Format(#[from] std::fmt::Error),
}

pub fn expand_env_placeholders(input: &str) -> Result<Cow<'_, str>, EnvPlaceholderError> {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(\\)?\$\{([A-Za-z_][A-Za-z0-9_]*)}").unwrap());

    if !RE.is_match(input) {
        return Ok(Cow::Borrowed(input));
    }

    let mut new_string = String::with_capacity(input.len());
    let mut last_match_end = 0;

    for caps in RE.captures_iter(input) {
        let match_whole = caps.get(0).unwrap();

        new_string.push_str(&input[last_match_end..match_whole.start()]);

        let is_escaped = caps.get(1).is_some();
        let name = &caps[2];

        if is_escaped {
            write!(new_string, "${{{name}}}")?;
        } else {
            let val = std::env::var(name)?;
            new_string.push_str(&val);
        }

        last_match_end = match_whole.end();
    }

    new_string.push_str(&input[last_match_end..]);

    Ok(Cow::Owned(new_string))
}
