use super::errors::*;

/// This should be called before calling any cli method or printing any output.
pub fn reset_signal_pipe_handler() -> Result<(), CliError> {
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
