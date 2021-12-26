use super::commit_occurrence::*;
use std::collections::{BTreeMap, BTreeSet};

pub fn generate(occurrences: &[CommitOccurrence]) {
    let mut contributors = Contributors::default();
    for (name, email) in occurrences
        .iter()
        .map(|commit| (&commit.original_name, &commit.original_email))
    {
        contributors.insert(name, email);
    }

    for (name, email, alternate_email) in contributors.to_mailmap() {
        if let Some(alternate) = alternate_email {
            println!("{} <{}> <{}>", name, email, alternate);
        } else {
            println!("{} <{}>", name, email);
        }
    }
}

#[derive(Debug, Clone)]
struct SetWithCanonical<V>((V, BTreeSet<V>));

impl<V: Copy + std::hash::Hash + Ord> SetWithCanonical<V> {
    fn new(value: V) -> Self {
        SetWithCanonical((value, BTreeSet::default()))
    }

    fn new_with_aliases<T: IntoIterator<Item = V>>(value: V, iter: T) -> Self {
        let mut result = SetWithCanonical((value, BTreeSet::default()));
        result.extend(iter);
        result
    }

    fn all(&self) -> BTreeSet<V> {
        let mut result = BTreeSet::from([self.primary()]);
        result.extend(self.secondary());
        result
    }

    fn primary(&self) -> V {
        self.0 .0
    }

    fn secondary(&self) -> &BTreeSet<V> {
        &self.0 .1
    }
}

impl<V: Ord> Extend<V> for SetWithCanonical<V> {
    fn extend<T: IntoIterator<Item = V>>(&mut self, iter: T) {
        for v in iter {
            if v != self.0 .0 {
                self.0 .1.insert(v);
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Contributor<'a> {
    name: SetWithCanonical<&'a str>,
    email: SetWithCanonical<&'a str>,
}

impl<'a> Contributor<'a> {
    fn emails(&self) -> BTreeSet<&str> {
        self.email.all()
    }

    fn names(&self) -> BTreeSet<&str> {
        self.name.all()
    }

    fn add_emails(&mut self, emails: BTreeSet<&'a str>) -> &mut Self {
        self.email.extend(emails);
        self
    }

    fn add_names(&mut self, names: BTreeSet<&'a str>) -> &mut Self {
        self.name.extend(names);
        self
    }
}

#[derive(Debug)]
struct Contributors<'a> {
    by_name: BTreeMap<String, SetWithCanonical<&'a str>>,
    by_email: BTreeMap<String, SetWithCanonical<&'a str>>,
}

impl<'a> Default for Contributors<'a> {
    fn default() -> Self {
        Contributors {
            by_name: BTreeMap::default(),
            by_email: BTreeMap::default(),
        }
    }
}

impl<'a> Contributors<'a> {
    fn to_mailmap(&self) -> Vec<(&str, &str, Option<&str>)> {
        let mut results = vec![];

        for contributor in self.to_contributors().values() {
            if contributor.email.secondary().is_empty() {
                results.push((
                    contributor.name.primary(),
                    contributor.email.primary(),
                    None,
                ));
            } else {
                for &email in contributor.email.secondary() {
                    results.push((
                        contributor.name.primary(),
                        contributor.email.primary(),
                        Some(email),
                    ));
                }
            }
        }

        results
    }

    fn insert(&mut self, name: &'a str, email: &'a str) -> &mut Self {
        self.by_name
            .entry(name.to_string())
            .and_modify(|existing| {
                existing.extend([email]);
            })
            .or_insert(SetWithCanonical::new(email));

        self.by_email
            .entry(email.to_string())
            .and_modify(|existing| {
                existing.extend([name]);
            })
            .or_insert(SetWithCanonical::new(name));

        self
    }

    fn len(&self) -> usize {
        self.to_contributors()
            .into_values()
            .collect::<Vec<_>>()
            .len()
    }

    fn get(&self, name: &str) -> Option<Contributor> {
        self.to_contributors().get(name).cloned()
    }

    fn to_contributors(&self) -> BTreeMap<&str, Contributor> {
        let mut results: BTreeMap<&str, Contributor> = BTreeMap::default();

        // Maintain a dictionary of aliased name -> canonical name
        let mut visited_names: BTreeMap<&str, &str> = BTreeMap::default();

        for (name, emails) in &self.by_name {
            let all_emails = emails.all();

            // For all emails for this contributor, build a set of all of their aliases
            let mut name_aliases = all_emails.iter().fold(BTreeSet::default(), |mut acc, em| {
                acc.extend(self.names_for_email(em));
                acc
            });
            // Remove the active name from the list of aliases
            name_aliases.remove(name.as_str());

            for name_alias in &name_aliases {
                // Maintain the dictionary of alias -> canonical name
                visited_names.insert(name_alias, name);
            }

            // the current name already has an entry
            if visited_names.contains_key(name.as_str()) {
                // Find the canonical name
                if let Some(&canonical_name) = visited_names.get(name.as_str()) {
                    // Find the Contributor based on the canonical name
                    if let Some(contributor) = results.get_mut(canonical_name) {
                        // Track all emails and aliases on the found contributor
                        contributor.add_emails(all_emails);
                        contributor.add_names(name_aliases);
                    }
                }
            } else {
                // no entry exists for this name
                results.insert(
                    name,
                    Contributor {
                        name: SetWithCanonical::new_with_aliases(name.as_str(), name_aliases),
                        email: emails.clone(),
                    },
                );
            }
        }

        results
    }

    fn names_for_email(&self, email: &str) -> BTreeSet<&str> {
        if let Some(results) = self.by_email.get(email) {
            results.all()
        } else {
            BTreeSet::default()
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_unique_insert() {
        let mut contributors = Contributors::default();
        contributors.insert("Test", "person@example.com");
        contributors.insert("Test", "person@example.com");
        assert_eq!(contributors.len(), 1);
        assert_eq!(
            contributors.get("Test").unwrap().emails(),
            BTreeSet::from(["person@example.com"])
        );
    }

    #[test]
    fn test_flatten_insert() {
        let mut contributors = Contributors::default();
        contributors.insert("Test", "person@example.com");
        contributors.insert("Test", "person+other@example.com");
        assert_eq!(contributors.len(), 1);
        assert_eq!(
            contributors.get("Test").unwrap().emails(),
            BTreeSet::from(["person@example.com", "person+other@example.com"])
        );
    }

    #[test]
    fn test_flatten_name_insert() {
        let mut contributors = Contributors::default();
        contributors.insert("Jane Doe", "jane@example.com");
        contributors.insert("Jane A Doe", "jane@example.com");
        assert_eq!(contributors.len(), 1);
        assert_eq!(
            contributors.get("Jane A Doe").unwrap().names(),
            BTreeSet::from(["Jane A Doe", "Jane Doe"])
        );
    }

    #[test]
    fn test_collapse_insert() {
        let mut contributors = Contributors::default();
        contributors.insert("Jane Doe", "jane@example.com");
        contributors.insert("Jane A Doe", "jane@other-example.com");
        contributors.insert("Jane A Doe", "jane@example.com");
        contributors.insert("JAD", "jane@other-example.com");
        assert_eq!(contributors.len(), 1);
        assert_eq!(
            contributors.get("JAD").unwrap().emails(),
            BTreeSet::from(["jane@example.com", "jane@other-example.com"])
        );
        assert_eq!(
            contributors.get("JAD").unwrap().names(),
            BTreeSet::from(["Jane Doe", "Jane A Doe", "JAD"])
        );
    }

    #[test]
    fn test_to_mailmap() {
        let mut contributors = Contributors::default();

        contributors.insert("Mike Burns", "mburns@thoughtbot.com");
        contributors.insert("Josh Clayton", "jclayton@thoughtbot.com");
        contributors.insert("Joshua Clayton", "jclayton+test@thoughtbot.com");
        contributors.insert("Josh Clayton", "jclayton+test@thoughtbot.com");
        contributors.insert("Joshua Clayton", "josh@thoughtbot.com");

        assert!(contributors.to_mailmap().contains(&(
            "Josh Clayton",
            "jclayton@thoughtbot.com",
            Some("jclayton+test@thoughtbot.com")
        )));

        assert!(contributors.to_mailmap().contains(&(
            "Josh Clayton",
            "jclayton@thoughtbot.com",
            Some("josh@thoughtbot.com")
        )));

        assert!(contributors
            .to_mailmap()
            .contains(&("Mike Burns", "mburns@thoughtbot.com", None)));
    }

    #[test]
    fn set_with_canonical_behavior() {
        let mut set: SetWithCanonical<usize> = SetWithCanonical::new(1);
        set.extend([1]);
        set.extend([1, 2]);
        set.extend([3]);
        assert_eq!(set.primary(), 1);
        assert_eq!(set.secondary(), &BTreeSet::from([2, 3]));
        assert_eq!(set.all(), BTreeSet::from([1, 2, 3]));
    }
}
