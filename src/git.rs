use git2::{Commit, Delta, Diff, DiffDelta, Repository};
use std::collections::BTreeMap;
use std::path::PathBuf;

pub fn retrieve_commits<'a>(repo: &'a Repository) -> Result<Vec<Commit<'a>>, git2::Error> {
    let mut revwalk = repo.revwalk()?;

    revwalk.set_sorting(git2::Sort::REVERSE | git2::Sort::TOPOLOGICAL)?;
    revwalk.push_head()?;

    let revwalk = revwalk
        .filter_map(Result::ok)
        .filter_map(|id| repo.find_commit(id).ok());

    Ok(revwalk.collect())
}

enum Operation {
    Add,
    Remove,
}

fn delta_operation(delta: &DiffDelta) -> Option<(PathBuf, Operation)> {
    match (delta.status(), delta.new_file().path()) {
        (Delta::Added, Some(path)) => Some((path.to_path_buf(), Operation::Add)),
        (Delta::Deleted, Some(path)) => Some((path.to_path_buf(), Operation::Remove)),
        (Delta::Modified, Some(path)) => Some((path.to_path_buf(), Operation::Add)),
        _ => None,
    }
}

pub fn build_churn(repo: &Repository, commits: &[Commit]) -> BTreeMap<PathBuf, usize> {
    commits
        .iter()
        .filter_map(|commit| build_diff(repo, commit).ok())
        .fold(BTreeMap::new(), |mut changes, diff| {
            for delta in diff.deltas() {
                match delta_operation(&delta) {
                    Some((path, Operation::Add)) => {
                        *changes.entry(path).or_insert(0) += 1;
                    }
                    Some((path, Operation::Remove)) => {
                        changes.remove(&path);
                    }
                    _ => (),
                }
            }

            changes
        })
}

fn build_diff<'a>(repo: &'a Repository, commit: &'a Commit) -> Result<Diff<'a>, git2::Error> {
    let parent_tree = if commit.parents().len() == 1 {
        let parent = commit.parent(0)?;
        Some(parent.tree()?)
    } else {
        None
    };
    let tree = commit.tree()?;

    repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), None)
}
