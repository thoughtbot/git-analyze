use chrono::prelude::*;
use std::collections::BTreeMap;
use std::marker::PhantomData;

pub type GroupedByWeek<T> = GroupedByDate<T, Week>;
pub type GroupedByMonth<T> = GroupedByDate<T, Month>;
pub type GroupedByQuarter<T> = GroupedByDate<T, Quarter>;
pub type GroupedByYear<T> = GroupedByDate<T, Year>;

pub trait Dated {
    fn occurred_on(&self) -> NaiveDate;
}

pub trait Period {
    fn beginning(date: &NaiveDate) -> Option<NaiveDate>;
    fn advance(date: &NaiveDate) -> Option<NaiveDate>;
}

pub struct Week;
pub struct Month;
pub struct Quarter;
pub struct Year;

impl Period for Week {
    fn beginning(date: &NaiveDate) -> Option<NaiveDate> {
        date_calculations::beginning_of_week(date)
    }

    fn advance(date: &NaiveDate) -> Option<NaiveDate> {
        date_calculations::next_week(date)
    }
}

impl Period for Month {
    fn beginning(date: &NaiveDate) -> Option<NaiveDate> {
        date_calculations::beginning_of_month(date)
    }

    fn advance(date: &NaiveDate) -> Option<NaiveDate> {
        date_calculations::next_month(date)
    }
}

impl Period for Quarter {
    fn beginning(date: &NaiveDate) -> Option<NaiveDate> {
        date_calculations::beginning_of_quarter(date)
    }

    fn advance(date: &NaiveDate) -> Option<NaiveDate> {
        date_calculations::next_quarter(date)
    }
}

impl Period for Year {
    fn beginning(date: &NaiveDate) -> Option<NaiveDate> {
        date_calculations::beginning_of_year(date)
    }

    fn advance(date: &NaiveDate) -> Option<NaiveDate> {
        date_calculations::next_year(date)
    }
}

pub struct GroupedByDate<T, P: Period> {
    records: BTreeMap<NaiveDate, T>,
    lock: PhantomData<P>,
}

impl<T, P: Period> IntoIterator for GroupedByDate<T, P> {
    type Item = (NaiveDate, T);
    type IntoIter = std::collections::btree_map::IntoIter<NaiveDate, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.records.into_iter()
    }
}

impl<'a, T, P: Period> IntoIterator for &'a GroupedByDate<T, P> {
    type Item = (&'a NaiveDate, &'a T);
    type IntoIter = std::collections::btree_map::Iter<'a, NaiveDate, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.records.iter()
    }
}

impl<T, P: Period> GroupedByDate<T, P> {
    pub fn map<U, F>(self, f: F) -> GroupedByDate<U, P>
    where
        F: Fn(T) -> U,
    {
        let mut result = BTreeMap::default();

        for (k, v) in self.into_iter() {
            result.insert(k, f(v));
        }

        GroupedByDate {
            records: result,
            lock: PhantomData,
        }
    }

    pub fn new<F, V>(mut records: Vec<V>, f: F) -> Self
    where
        V: Dated,
        F: Fn(Vec<V>) -> T,
    {
        let mut result = BTreeMap::default();

        let today = Local::now().naive_local().date();
        let final_date = records
            .iter()
            .max_by_key(|x| x.occurred_on())
            .map(|x| x.occurred_on())
            .unwrap_or(today);
        if let Some(earliest) = records.iter().map(|v| v.occurred_on()).min() {
            let mut current_date = P::beginning(&earliest).unwrap();

            while current_date <= final_date {
                let next_date = P::advance(&current_date).unwrap();
                result.insert(
                    current_date,
                    f(records
                        .drain_filter(|r| {
                            r.occurred_on() >= current_date && r.occurred_on() < next_date
                        })
                        .collect()),
                );

                current_date = next_date;
            }
        }

        GroupedByDate {
            records: result,
            lock: PhantomData,
        }
    }
}
