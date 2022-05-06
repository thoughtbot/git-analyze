use git2::{Commit, Repository};

pub fn retrieve_commits<'a>(repo: &'a Repository) -> Result<Vec<Commit<'a>>, git2::Error> {
    let mut revwalk = repo.revwalk()?;

    revwalk.set_sorting(git2::Sort::REVERSE | git2::Sort::TOPOLOGICAL)?;
    revwalk.push_head()?;

    let revwalk = revwalk
        .filter_map(Result::ok)
        .filter_map(|id| repo.find_commit(id).ok());

    Ok(revwalk.collect())
}
