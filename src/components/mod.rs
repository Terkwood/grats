mod app;
mod config;
mod daily;
mod entry_buttons_view;
mod history_view;
mod nav;

pub use app::App;
pub use config::Config;
pub use daily::Daily;
pub use entry_buttons_view::EntryButtonsView;
pub use history_view::HistoryView;
pub use nav::Nav;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Page {
    Daily,
    History,
    Config,
}

impl Default for Page {
    fn default() -> Self {
        Page::Daily
    }
}
