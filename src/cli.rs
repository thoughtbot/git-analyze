pub mod errors;
mod flags;
pub mod unix;

use super::commit_occurrence::*;
use super::mailmap;
use crate::grouped_by_date::{GroupedByDate, Period, Quarter};
pub use errors::*;
use flags::*;
use git2::Repository;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use structopt::StructOpt;
pub use unix::*;

pub fn run() -> Result<(), CliError> {
    let flags = Flags::from_args();

    let repo = Repository::open(".")?;
    let mailmap = repo.mailmap()?;
    let mut revwalk = repo.revwalk()?;

    revwalk.set_sorting(git2::Sort::REVERSE | git2::Sort::TOPOLOGICAL)?;

    revwalk.push_head()?;

    let revwalk = revwalk
        .filter_map(Result::ok)
        .filter_map(|id| repo.find_commit(id).ok());

    let occurrences = revwalk
        .map(|commit| {
            CommitOccurrence::build(
                commit.clone(),
                commit.author(),
                mailmap.resolve_signature(&commit.author()).ok(),
            )
        })
        .collect::<Vec<_>>();

    match flags.cmd {
        None | Some(Command::Overview) => {
            print_overview(&occurrences);
        }

        Some(Command::TeamHistory { verbose }) => {
            print_periodic_team_changes(Quarter, occurrences, verbose);
        }
        Some(Command::OffHours { verbose }) => {
            print_periodic_off_hours_occurrences(Quarter, occurrences, verbose);
        }

        Some(Command::GenerateMailmap) => mailmap::generate(&occurrences),
    }

    Ok(())
}

#[allow(dead_code)]
fn print_commit(commit: &CommitOccurrence) {
    println!("commit {}", commit.id);
    println!("Author: {} <{}>", commit.name, commit.email);
    println!(
        "Date:   {}",
        commit.at.format("%Y-%m-%d %H:%M:%S %z").to_string(),
    );
    println!();
}

fn print_overview(occurrences: &[CommitOccurrence]) {
    println!("Total commits: {}", occurrences.len());
    println!("First commit: {:?}", occurrences.first().unwrap().at);
    println!(
        "Unique committers: {:?}",
        occurrences.iter().unique_by(|c| &c.name).count()
    );
    println!(
        "Recent committers: {:?}",
        occurrences
            .iter()
            .filter(|c| c.at > chrono::Local::now() - chrono::Duration::weeks(26))
            .unique_by(|c| &c.name)
            .count()
    );

    println!("Top 10 committers:");
    for (author, count) in contribution_counts(occurrences.to_vec())
        .into_iter()
        .sorted_by_key(|c| c.1 as isize * -1)
        .take(10)
    {
        println!("* {} {}", author, count);
    }
}

fn contribution_counts(mut occurrences: Vec<CommitOccurrence>) -> BTreeMap<String, usize> {
    let authors = occurrences
        .iter()
        .map(|c| c.name.clone())
        .collect::<BTreeSet<_>>();
    let mut result = BTreeMap::default();

    for author in authors {
        result.insert(
            author.clone(),
            occurrences.drain_filter(|o| o.name == author).count(),
        );
    }

    result
}

fn grouped_commit_occurrences<P: Period>(
    _: P,
    occurrences: Vec<CommitOccurrence>,
) -> GroupedByDate<Vec<CommitOccurrence>, P> {
    GroupedByDate::new(occurrences, |v| v)
}

fn print_periodic_off_hours_occurrences<P: Period>(
    period: P,
    occurrences: Vec<CommitOccurrence>,
    verbose: bool,
) {
    let grouped_by_period = grouped_commit_occurrences(period, occurrences);

    for (date, occ) in grouped_by_period {
        let night_or_weekend_occurrences = occ.iter().filter(|o| o.is_night() || o.is_weekend());
        let count = night_or_weekend_occurrences.clone().count();

        let total = occ.iter().len();
        let percentage = count as f32 / total as f32;

        let authors = night_or_weekend_occurrences
            .map(|c| c.name.clone())
            .collect::<BTreeSet<_>>();

        println!(
            "{:?}: {:.2}% ({} of {})",
            date,
            percentage * 100.0,
            count,
            total
        );

        if verbose {
            for author in authors {
                println!("  * {}", author);
            }
        }
    }
}

fn print_periodic_team_changes<P: Period>(
    period: P,
    occurrences: Vec<CommitOccurrence>,
    verbose: bool,
) {
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

        if verbose {
            for author in &current_authors {
                println!("  * {}", author);
            }
        }

        prior_authors = current_authors;
    }
}
