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

#[cfg(test)]
mod test {

    use super::*;
    use crate::model::*;

    const LOCAL_OFFSET_SECONDS: i32 = 240;

    fn make_time(days: u32) -> DateTime<Utc> {
        Utc.ymd(2020, 08, days + 1).and_hms(0, 0, 0)
    }

    fn list_with(size: u32) -> GratitudeList {
        let emoji: Emoji = Emoji("🤩".to_string());

        let mut entries = vec![];
        for i in 0..size {
            entries.push(Entry {
                emoji: emoji.clone(),
                text: "yes".to_string(),
                time: UtcMillis(make_time(i).timestamp_millis() as u64),
            });
        }
        GratitudeList { entries }
    }
    #[test]
    fn test_grouping() {
        let size = 10;
        let list = list_with(size);
        let history = History::from(&list, FixedOffset::west(LOCAL_OFFSET_SECONDS));

        assert_eq!(history.days.len(), size as usize);
        for day in history.days {
            assert_eq!(day.gratitude_list.entries.len(), 1)
        }
        assert_eq!(history.offset, FixedOffset::west(LOCAL_OFFSET_SECONDS))
    }

    #[test]
    fn test_from_order() {
        let size = 3;
        let list = list_with(3);
        let history = History::from(&list, FixedOffset::west(LOCAL_OFFSET_SECONDS));

        for i in 0..(size - 1) {
            let this_date = history.days[i].date;
            let next_date = history.days[i + 1].date;
            assert!(this_date > next_date);
        }
    }
}
