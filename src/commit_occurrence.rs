use super::grouped_by_date::Dated;
use chrono::{DateTime, Datelike, FixedOffset, TimeZone, Timelike, Weekday};
use git2::{Commit, Oid, Signature};

#[derive(Debug, Clone)]
pub struct CommitOccurrence {
    pub name: String,
    pub email: String,
    pub original_name: String,
    pub original_email: String,
    pub id: Oid,
    pub at: DateTime<FixedOffset>,
}

impl Dated for CommitOccurrence {
    fn occurred_on(&self) -> chrono::NaiveDate {
        self.at.naive_utc().date()
    }
}

impl CommitOccurrence {
    pub fn build(commit: Commit, author: Signature, resolved_author: Option<Signature>) -> Self {
        let time = author.when();
        let offset = if time.offset_minutes() < 0 {
            FixedOffset::east(time.offset_minutes() * 60)
        } else {
            FixedOffset::west(time.offset_minutes() * 60)
        };

        let t = offset.timestamp(time.seconds(), 0);

        let original_name = author.name().unwrap_or("").to_string();
        let original_email = author.email().unwrap_or("").to_string();

        let name = resolved_author.as_ref().and_then(|a| a.name());
        let email = resolved_author.as_ref().and_then(|a| a.email());

        CommitOccurrence {
            name: name.unwrap_or(&original_name).to_string(),
            email: email.unwrap_or(&original_email).to_string(),
            original_name,
            original_email,
            id: commit.id(),
            at: t,
        }
    }

    pub fn is_night(&self) -> bool {
        self.at.hour() > 19 || self.at.hour() <= 7
    }

    pub fn is_weekend(&self) -> bool {
        self.at.weekday() == Weekday::Sat || self.at.weekday() == Weekday::Sun
    }
}
