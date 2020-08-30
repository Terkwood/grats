use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GratitudeList {
    entries: Vec<Entry>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq)]
pub struct Entry {
    pub epoch_millis_utc: u64,
    pub entry_type: EntryType,
    pub text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq)]
pub struct EntryType {
    pub name: String,
    pub emoji: String,
}

impl GratitudeList {
    pub fn empty() -> Self {
        Self { entries: vec![] }
    }

    pub fn today(&self, now: UtcMillis, offset: FixedOffset) -> Self {
        let local_now_date = Utc
            .timestamp_millis(now.0 as i64)
            .with_timezone(&offset)
            .date();
        let entries = self
            .entries
            .iter()
            .filter(|entry| {
                Utc.timestamp_millis(entry.epoch_millis_utc as i64)
                    .with_timezone(&offset)
                    .date()
                    == local_now_date
            })
            .cloned()
            .collect();
        GratitudeList { entries }
    }

    pub fn add(&mut self, entry: Entry) {
        self.entries.insert(0, entry)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UtcMillis(pub u64);
