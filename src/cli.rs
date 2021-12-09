use super::commit_occurrence::*;
use git2::Error;
use git2::Repository;

pub fn run() -> Result<(), Error> {
    let repo = Repository::open(".")?;
    let mut revwalk = repo.revwalk()?;

    revwalk.set_sorting(git2::Sort::TOPOLOGICAL)?;

    revwalk.push_head()?;

    let revwalk = revwalk
        .filter_map(Result::ok)
        .filter_map(|id| repo.find_commit(id).ok());

    for commit in revwalk {
        print_commit(CommitOccurrence::build(&commit, &commit.author()));
    }

    Ok(())
}

fn print_commit(commit: CommitOccurrence) {
    println!("commit {}", commit.id);
    println!("Author: {} <{}>", commit.name, commit.email);
    println!(
        "Date:   {}",
        commit.at.format("%Y-%m-%d %H:%M:%S %z").to_string(),
    );
    println!();
}
