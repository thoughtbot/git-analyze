use super::grouped_by_date::Dated;
use chrono::{DateTime, FixedOffset, TimeZone};
use git2::{Commit, Oid, Signature};

#[derive(Debug, Clone)]
pub struct CommitOccurrence {
    pub name: String,
    pub email: String,
    pub id: Oid,
    pub at: DateTime<FixedOffset>,
}

impl Dated for CommitOccurrence {
    fn occurred_on(&self) -> chrono::NaiveDate {
        self.at.naive_utc().date()
    }
}

impl CommitOccurrence {
    pub fn build(commit: Commit, author: Signature) -> Self {
        let time = author.when();
        let offset = if time.offset_minutes() < 0 {
            FixedOffset::east(time.offset_minutes() * 60)
        } else {
            FixedOffset::west(time.offset_minutes() * 60)
        };

        let t = offset.timestamp(time.seconds(), 0);

        CommitOccurrence {
            name: author.name().unwrap_or("").to_string(),
            email: author.email().unwrap_or("").to_string(),
            id: commit.id(),
            at: t,
        }
    }
}