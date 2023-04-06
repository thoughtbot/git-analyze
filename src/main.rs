use git_analyze::cli;
use tokio;

#[tokio::main]
async fn main() {
    std::process::exit(match run().await {
        Ok(_) => 0,
        Err(cli::CliError::GitError(e)) => {
            eprintln!("git-analyze Error: {}", e.message());
            1
        }

        Err(cli::CliError::IoError(e)) => {
            eprintln!("git-analyze Error: {}", e);
            1
        }
    })
}

async fn run() -> Result<(), cli::CliError> {
    cli::reset_signal_pipe_handler()?;
    cli::run().await
}
