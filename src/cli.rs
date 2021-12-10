use super::commit_occurrence::*;
use crate::grouped_by_date::{GroupedByDate, Period, Quarter};
use chrono::{DateTime, FixedOffset};
use git2::Error;
use git2::Repository;
use itertools::*;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub fn run() -> Result<(), Error> {
    let repo = Repository::open(".")?;
    let mut revwalk = repo.revwalk()?;

    revwalk.set_sorting(git2::Sort::REVERSE | git2::Sort::TOPOLOGICAL)?;

    revwalk.push_head()?;

    let revwalk = revwalk
        .filter_map(Result::ok)
        .filter_map(|id| repo.find_commit(id).ok());

    let occurrences = revwalk
        .map(|commit| CommitOccurrence::build(commit.clone(), commit.author()))
        .collect::<Vec<_>>();

    print_authorship_timelines(&occurrences);

    print_periodic_team_changes(Quarter, occurrences);

    Ok(())
}

fn commits_by_author(
    commits: &[CommitOccurrence],
) -> BTreeMap<String, (DateTime<FixedOffset>, DateTime<FixedOffset>)> {
    commits
        .iter()
        .sorted_by_key(|c| &c.name)
        .group_by(|c| c.name.clone())
        .into_iter()
        .map(|(k, v)| {
            let mut dates = v.map(|c| c.at).collect::<Vec<_>>();
            dates.sort();

            (
                k,
                (
                    dates.first().unwrap().clone(),
                    dates.last().unwrap().clone(),
                ),
            )
        })
        .collect::<BTreeMap<_, _>>()
}

fn print_commit(commit: &CommitOccurrence) {
    println!("commit {}", commit.id);
    println!("Author: {} <{}>", commit.name, commit.email);
    println!(
        "Date:   {}",
        commit.at.format("%Y-%m-%d %H:%M:%S %z").to_string(),
    );
    println!();
}

fn print_authorship_timelines(occurrences: &[CommitOccurrence]) {
    for (author, (start, end)) in commits_by_author(occurrences) {
        println!(
            "{:?} {:?} {:?} ({})",
            author,
            start,
            end,
            (end - start).num_days(),
        );
    }
}

fn grouped_commit_occurrences<P: Period>(
    _: P,
    occurrences: Vec<CommitOccurrence>,
) -> GroupedByDate<Vec<CommitOccurrence>, P> {
    GroupedByDate::new(occurrences, |v| v)
}

fn print_periodic_team_changes<P: Period>(period: P, occurrences: Vec<CommitOccurrence>) {
    let grouped_by_period = grouped_commit_occurrences(period, occurrences);

    let mut prior_authors: BTreeSet<String> = BTreeSet::new();
    for (date, occ) in grouped_by_period {
        let current_authors = occ.iter().map(|c| c.name.clone()).collect::<BTreeSet<_>>();
        let repeated_authors = prior_authors.intersection(&current_authors).count();
        let new_authors = current_authors.difference(&prior_authors).count();
        let retired_authors = prior_authors.difference(&current_authors).count();
        println!(
            "{:?} (total: {}, same: {}, +{}, -{})",
            date,
            current_authors.len(),
            repeated_authors,
            new_authors,
            retired_authors
        );

        prior_authors = current_authors;
    }
}