mod app;
mod daily;

pub use app::App;
pub use daily::Daily;

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

impl Page {
    pub fn order() -> Vec<Page> {
        vec![Page::Daily, Page::History, Page::Config]
    }

    fn next(self) -> Self {
        let self_pos = order_pos(self);
        let next_pos = (self_pos + 1) % Self::order().len();
        Page::order()[next_pos]
    }

    fn prev(self) -> Page {
        let self_pos = order_pos(self);
        let len = Page::order().len();
        let prev_pos = (self_pos + len - 1) % len;
        Page::order()[prev_pos]
    }
}

fn order_pos(page: Page) -> usize {
    Page::order()
        .iter()
        .enumerate()
        .find(|(_pos, that)| **that == page)
        .map(|(pos, _)| pos)
        .unwrap_or_default()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_next() {
        assert_eq!(Page::Daily.next(), Page::History);
        assert_eq!(Page::History.next(), Page::Config);
        assert_eq!(Page::Config.next(), Page::Daily);
    }

    #[test]
    fn test_prev() {
        assert_eq!(Page::Daily.prev(), Page::Config);
        assert_eq!(Page::History.prev(), Page::Daily);
        assert_eq!(Page::Config.prev(), Page::History);
    }
}
