mod entry_button;
pub mod history;

pub use entry_button::EntryButtonCollection;
pub use history::History;

use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Ord, Eq, PartialOrd)]
pub struct GratitudeList {
    pub entries: Vec<Entry>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq)]
pub struct Entry {
    pub time: UtcMillis,
    pub emoji: Emoji,
    pub text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq)]
pub struct Emoji(pub String);

pub enum DefaultEmoji {
    Sun,
    RedHeart,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize)]
pub struct UtcMillis(pub u64);

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
                Utc.timestamp_millis(entry.time.0 as i64)
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

impl DefaultEmoji {
    pub fn all() -> Vec<DefaultEmoji> {
        vec![DefaultEmoji::Sun, DefaultEmoji::RedHeart]
    }

    pub fn instance(&self) -> Emoji {
        match self {
            DefaultEmoji::Sun => Emoji("🌞".to_string()),
            DefaultEmoji::RedHeart => Emoji("❤️".to_string()),
        }
    }
}
