use super::commit_occurrence::*;
use std::collections::BTreeSet;

pub fn generate(occurrences: &[CommitOccurrence]) {
    for (name, email) in occurrences
        .iter()
        .map(|commit| (commit.original_name.clone(), commit.original_email.clone()))
        .collect::<BTreeSet<_>>()
    {
        println!("{} <{}>", name, email);
    }
}
