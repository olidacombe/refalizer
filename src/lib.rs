//! Make sure your branch names are good to drive.
//!
//! Substitutes whitespace with underscore, abides by [the rules](https://git-scm.com/docs/git-check-ref-format).
//!
//! ## Usage
//!
//! ```bash
//! refalizer many "disjoint (ish)" Arguments
//! ```
//! would return `many_disjoint_(ish)_Arguments`

#![deny(clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::must_use_candidate)]

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RefError {
    #[error("no valid output possible")]
    NoValidOutput,
}

macro_rules! unprefix {
    ($subject:ident, $prefix:expr) => {
        if $subject.starts_with($prefix) {
            return standardize(
                $subject
                    .strip_prefix($prefix)
                    .map(String::from)
                    .unwrap_or_default(),
            );
        }
    };
}

macro_rules! unsuffix {
    ($subject:ident, $suffix:expr) => {
        if $subject.ends_with($suffix) {
            return standardize(
                $subject
                    .strip_suffix($suffix)
                    .map(String::from)
                    .unwrap_or_default(),
            );
        }
    };
}

macro_rules! swap {
    ($subject:ident, $before:expr, $after:expr) => {
        if $subject.contains($before) {
            return standardize($subject.replace($before, $after));
        }
    };
}

fn standardize(input: String) -> String {
    unsuffix!(input, ".lock");
    unsuffix!(input, "/");
    unsuffix!(input, ".");
    unprefix!(input, "/");
    swap!(input, " ", "_");
    swap!(input, ":", "_");
    swap!(input, "^", "_");
    swap!(input, "~", "_");
    swap!(input, "?", "_");
    swap!(input, "*", "_");
    swap!(input, "[", "_");
    swap!(input, "\\", "_");
    swap!(input, "__", "_");
    swap!(input, "..", ".");
    swap!(input, "//", "/");
    swap!(input, "\x7f", "_");
    swap!(input, "\x20", "_");
    swap!(input, "@{", "_");
    swap!(input, "./", "/");
    swap!(input, "/.", "/");
    input
}

pub fn to_branch_name<S: AsRef<str> + std::fmt::Debug>(input: &S) -> Result<String, RefError> {
    let output = standardize(input.as_ref().to_string());
    if output.is_empty() {
        return Err(RefError::NoValidOutput);
    }
    Ok(output.to_string())
}
