use super::*;
use serde_derive::{Deserialize, Serialize};

const MAX_USER_BUTTONS: u8 = 4;

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct EntryButtonCollection {
    pub user_emojis: Vec<Emoji>,
}

const ALLOWED_EMOJIS: &[&str] = &[
    "🥳",
    "🤩",
    "🥰",
    "😁",
    "😂",
    "🤣",
    "😆",
    "🌈",
    "🌌",
    "🍕",
    "🍎",
    "🍏",
    "🍍",
    "🍒",
    "🍜",
    "🍅",
    "🍔",
    "🌊",
    "🌟",
    "⭐",
    "🌠",
    "👪",
    "💑",
    "🏁",
    "🏡",
    "👨‍🌾",
    "🎲",
    "🌳",
    "🎄",
    "🐤",
    "🐶",
    "😸",
    "👩‍⚕️",
    "💘",
    "💜",
    "🧡",
    "💚",
    "💙",
    "💛",
    "💓",
    "💕",
    "💞",
    "💖",
    "🎼",
    "🎹",
    "🎵",
    "🎶",
    "🎸",
    "🎺",
    "🎷",
    "🥁",
    "🎨",
    "🖼",
    "👩‍🎨",
    "🏅",
    "🏆",
    "📕",
    "📚",
    "🏫",
    "🏙",
    "🌆",
    "🔥",
    "🎆",
    "🌎",
    "🌍",
    "🌏",
    "🏞",
];

impl EntryButtonCollection {
    pub fn empty() -> Self {
        Self {
            user_emojis: vec![],
        }
    }

    pub fn all(&self) -> Vec<Emoji> {
        let mut r = self.user_emojis.clone();
        for d in DefaultEmoji::all() {
            r.insert(0, d.instance())
        }
        r
    }

    pub fn free_user_buttons(&self) -> u8 {
        MAX_USER_BUTTONS - (self.all().len() - DefaultEmoji::all().len()) as u8
    }

    pub fn allowed_emojis(&self) -> Vec<String> {
        ALLOWED_EMOJIS
            .iter()
            .map(|s| s.to_string())
            .filter(|e| !self.user_emojis.contains(&Emoji(e.to_string())))
            .collect()
    }

    pub fn add(&mut self, button: Emoji) -> ButtonAddResult {
        if (self.user_emojis.len() as u8) < MAX_USER_BUTTONS {
            self.user_emojis.push(button);
            ButtonAddResult::Ok
        } else {
            ButtonAddResult::NotAdded
        }
    }

    pub fn delete(&mut self, button: &Emoji) {
        self.user_emojis.retain(|b| b != button)
    }
}

pub enum ButtonAddResult {
    Ok,
    NotAdded,
}
