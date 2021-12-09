use chrono::{DateTime, FixedOffset, TimeZone};
use git2::{Commit, Oid, Signature};

pub struct CommitOccurrence<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub id: Oid,
    pub at: DateTime<FixedOffset>,
}

impl<'a> CommitOccurrence<'a> {
    pub fn build(commit: &'a Commit, author: &'a Signature) -> Self {
        let time = author.when();
        let offset = if time.offset_minutes() < 0 {
            FixedOffset::east(time.offset_minutes() * 60)
        } else {
            FixedOffset::west(time.offset_minutes() * 60)
        };

        let t = offset.timestamp(time.seconds(), 0);

        CommitOccurrence {
            name: author.name().unwrap_or(""),
            email: author.email().unwrap_or(""),
            id: commit.id(),
            at: t,
        }
    }
}
