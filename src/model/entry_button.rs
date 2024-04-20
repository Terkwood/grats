use super::*;
use serde_derive::{Deserialize, Serialize};

const MAX_BUTTONS: u8 = 6;

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct EntryButtonCollection {
    pub emojis: Vec<Emoji>,
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
    "🌞",
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
    "🏞",
    "🍁",
    "🌴",
    "🌾",
    "🌺",
    "🌻",
    "🌼",
    "🌷",
    "🌱",
    "🍃",
    "🦚",
    "🌘",
    "🐤",
    "🐶",
    "😸",
    "🦜",
    "🦀",
    "👩‍⚕️",
    "🧘",
    "❤",
    "💜",
    "🧡",
    "💚",
    "💙",
    "💛",
    "💘",
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
    "⚧",
    "🔒"
];

impl EntryButtonCollection {
    pub fn new() -> Self {
        Self {
            emojis: DefaultEmoji::all().iter().map(|e| e.instance()).collect(),
        }
    }

    pub fn all(&self) -> Vec<Emoji> {
        self.emojis.clone()
    }

    pub fn free_user_buttons(&self) -> u8 {
        MAX_BUTTONS - self.all().len() as u8
    }

    pub fn allowed_emojis(&self) -> Vec<String> {
        ALLOWED_EMOJIS
            .iter()
            .map(|s| s.to_string())
            .filter(|e| !self.emojis.contains(&Emoji(e.to_string())))
            .collect()
    }

    pub fn add(&mut self, button: Emoji) -> ButtonAddResult {
        if (self.emojis.len() as u8) < MAX_BUTTONS {
            self.emojis.push(button);
            ButtonAddResult::Ok
        } else {
            ButtonAddResult::NotAdded
        }
    }

    pub fn delete(&mut self, button: &Emoji) {
        self.emojis.retain(|b| b != button)
    }
}

pub enum ButtonAddResult {
    Ok,
    NotAdded,
}
