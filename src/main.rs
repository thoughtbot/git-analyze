use git_analyze::cli;

fn main() {
    reset_signal_pipe_handler();
    cli::run();
}
/// This should be called before calling any cli method or printing any output.
fn reset_signal_pipe_handler() -> std::io::Result<()> {
    #[cfg(target_family = "unix")]
    {
        use nix::sys::signal;

        unsafe {
            signal::signal(signal::Signal::SIGPIPE, signal::SigHandler::SigDfl)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        }
    }

    Ok(())
}
