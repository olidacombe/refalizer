use thiserror::Error;

#[derive(Error, Debug)]
pub enum RefError {
    #[error("no valid output possible")]
    NoValidOutput,
}

pub fn to_branch_name<S: AsRef<str> + std::fmt::Debug>(input: S) -> Result<String, RefError> {
    dbg!(input);
    Ok("yolo".to_string())
}
