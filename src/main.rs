use chrono::NaiveDateTime;
use git2::Error;
use git2::{Commit, Repository, Time};
use std::str;

fn run() -> Result<(), Error> {
    let repo = Repository::open(".")?;
    let mut revwalk = repo.revwalk()?;

    revwalk.set_sorting(git2::Sort::TOPOLOGICAL)?;

    revwalk.push_head()?;

    let revwalk = revwalk
        .filter_map(Result::ok)
        .filter_map(|id| repo.find_commit(id).ok());

    for commit in revwalk {
        print_commit(&commit);
    }

    Ok(())
}

fn print_commit(commit: &Commit) {
    println!("commit {}", commit.id());

    if commit.parents().len() > 1 {
        print!("Merge:");
        for id in commit.parent_ids() {
            print!(" {:.8}", id);
        }
        println!();
    }

    let author = commit.author();
    println!("Author: {}", author);
    print_time(&author.when(), "Date:   ");
    println!();

    for line in String::from_utf8_lossy(commit.message_bytes()).lines() {
        println!("    {}", line);
    }
    println!();
}

fn print_time(time: &Time, prefix: &str) {
    let (offset, sign) = match time.offset_minutes() {
        n if n < 0 => (-n, '-'),
        n => (n, '+'),
    };
    let (hours, minutes) = (offset / 60, offset % 60);
    let t = NaiveDateTime::from_timestamp(time.seconds() + (time.offset_minutes() as i64) * 60, 0);

    println!(
        "{}{} {}{:02}{:02}",
        prefix,
        t.format("%Y-%m-%d %H:%M:%S").to_string(),
        sign,
        hours,
        minutes
    );
}

fn main() {
    reset_signal_pipe_handler();
    run();
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
