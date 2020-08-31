use super::Page;
use crate::model::*;
use crate::repo::*;
use yew::prelude::*;

pub struct App {
    page: Page,
    gratitude_list_repo: GratitudeListRepo,
    gratitude_list: GratitudeList,

    entry_buttons_repo: EntryButtonsRepo,
    entry_buttons: EntryButtonCollection,

    nav_state: NavState,
}

pub enum Msg {
    AddEntry(Entry),
    NavigateTo(Page),
    ShowNav(bool),
    AddEntryButton(Emoji),
    DeleteEntryButton(Emoji),
}

#[derive(PartialEq)]
pub enum NavState {
    Visible,
    Hidden,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let nav_to = Some(link.callback(|page| Msg::NavigateTo(page)));
        let show_nav = Some(link.callback(|b| Msg::ShowNav(b)));

        let gratitude_list_repo = GratitudeListRepo::new();
        let gratitude_list = gratitude_list_repo.read();

        let entry_buttons_repo = EntryButtonsRepo::new();
        let entry_buttons = entry_buttons_repo.read();

        Self {
            page: Page::Daily,
            nav_state: NavState::Visible,
            gratitude_list_repo,
            gratitude_list,
            entry_buttons_repo,
            entry_buttons,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddEntry(entry) => {
                self.gratitude_list.add(entry);
                self.gratitude_list_repo.save(&self.gratitude_list)
            }
            Msg::NavigateTo(page) => self.page = page,
            Msg::ShowNav(b) => {
                if b {
                    self.nav_state = NavState::Visible
                } else {
                    self.nav_state = NavState::Hidden
                }
            }
            Msg::AddEntryButton(emoji) => {
                self.entry_buttons.add(emoji);
                self.entry_buttons_repo.save(&self.entry_buttons)
            }
            Msg::DeleteEntryButton(emoji) => {
                self.entry_buttons.delete(&emoji);
                self.entry_buttons_repo.save(&self.entry_buttons);
            }
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        todo!()
    }
}
