#[derive(Debug)]
pub enum CliError {
    IoError(std::io::Error),
    GitError(git2::Error),
}

impl From<std::io::Error> for CliError {
    fn from(error: std::io::Error) -> Self {
        CliError::IoError(error)
    }
}

impl From<git2::Error> for CliError {
    fn from(error: git2::Error) -> Self {
        CliError::GitError(error)
    }
}
