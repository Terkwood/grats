use super::*;
use crate::model::*;
use crate::repo::*;
use crate::time::{js_local_offset, js_utc_now};
use yew::prelude::*;

pub struct App {
    page: Page,
    gratitude_list_repo: GratitudeListRepo,
    gratitude_list: GratitudeList,

    entry_buttons_repo: EntryButtonsRepo,
    entry_buttons: EntryButtonCollection,

    nav_state: NavState,

    nav_to: Option<Callback<Page>>,
    show_nav: Option<Callback<bool>>,

    add_entry: Option<Callback<Entry>>,

    add_entry_button: Option<Callback<Emoji>>,
    del_entry_button: Option<Callback<Emoji>>,
    reset_entry_buttons: Option<Callback<()>>,
}

pub enum Msg {
    AddEntry(Entry),
    NavigateTo(Page),
    ShowNav(bool),
    AddEntryButton(Emoji),
    DeleteEntryButton(Emoji),
    ResetEntryButtons,
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

        let add_entry = Some(link.callback(|entry| Msg::AddEntry(entry)));

        let add_entry_button = Some(link.callback(|emoji| Msg::AddEntryButton(emoji)));
        let del_entry_button = Some(link.callback(|emoji| Msg::DeleteEntryButton(emoji)));
        let reset_entry_buttons = Some(link.callback(|_| Msg::ResetEntryButtons));

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
            nav_to,
            show_nav,
            add_entry,
            add_entry_button,
            del_entry_button,
            reset_entry_buttons,
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
            Msg::ResetEntryButtons => {
                self.entry_buttons = EntryButtonCollection::new();
                self.entry_buttons_repo.save(&self.entry_buttons);
            }
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            <main>{
                match self.page {
                    Page::Daily => self.view_daily(),
                    Page::History => self.view_history(),
                    Page::Config => self.view_config(),
                }
            }</main>
            { self.view_nav() }
            </>
        }
    }
}

impl App {
    fn view_daily(&self) -> Html {
        html! {
            <Daily
                gratitude_list={self.gratitude_list.today(js_utc_now(), js_local_offset())}
                add_entry={self.add_entry.as_ref().expect("add entry cb")}
                entry_buttons={self.entry_buttons.clone()}
                show_nav={self.show_nav.as_ref().expect("show nav cb")}
            />
        }
    }

    fn view_history(&self) -> Html {
        html! {
            <HistoryView
                gratitude_list={self.gratitude_list.clone()}
            />
        }
    }

    fn view_config(&self) -> Html {
        html! {
            <Config
                entry_buttons={self.entry_buttons.clone()}
                gratitude_list={self.gratitude_list.clone()}
                add_entry_button={self.add_entry_button.as_ref().expect("add inv button cb")}
                del_entry_button={self.del_entry_button.as_ref().expect("del button cb")}
                reset_entry_buttons={self.reset_entry_buttons.as_ref().expect("reset buttons cb")}
            />
        }
    }

    fn view_nav(&self) -> Html {
        if self.nav_state == NavState::Visible {
            html! {
                <Nav
                    page={self.page}
                    nav_to={self.nav_to.as_ref().expect("nav cb")}
                />
            }
        } else {
            html! { <></> }
        }
    }
}
