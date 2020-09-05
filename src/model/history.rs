use crate::model::GratitudeList;
use chrono::prelude::*;
use group_by::group_by;

#[derive(Debug)]
pub struct History {
    pub days: Vec<Day>,
    pub offset: FixedOffset,
}

#[derive(Ord, Eq, PartialOrd, PartialEq, Debug)]
pub struct Day {
    pub date: Date<FixedOffset>,
    pub gratitude_list: GratitudeList,
}

impl History {
    pub fn from(list: &GratitudeList, offset: FixedOffset) -> Self {
        let mut days: Vec<Day> = group_by(&list.entries, |mr| {
            Utc.timestamp_millis(mr.time.0 as i64)
                .with_timezone(&offset)
                .date()
        })
        .iter()
        .map(|d| {
            let entries = d.1.iter().map(|&i| i.clone()).collect();
            Day {
                date: d.0,
                gratitude_list: GratitudeList { entries },
            }
        })
        .collect();
        days.sort_unstable();
        days.reverse();
        Self { days, offset }
    }
}
