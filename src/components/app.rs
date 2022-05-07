use super::*;
use crate::model::*;
use crate::repo::*;
use crate::time::{js_local_offset, js_utc_now};
use yew::prelude::*;
use yew::Context;

pub struct App {
    page: Page,
    gratitude_list_repo: GratitudeListRepo,
    gratitude_list: GratitudeList,

    entry_buttons_repo: EntryButtonsRepo,
    entry_buttons: EntryButtonCollection,

    nav_to: Option<Callback<Page>>,

    add_entry: Option<Callback<Entry>>,

    add_entry_button: Option<Callback<Emoji>>,
    del_entry_button: Option<Callback<Emoji>>,
    reset_entry_buttons: Option<Callback<()>>,
}

pub enum Msg {
    AddEntry(Entry),
    NavigateTo(Page),
    AddEntryButton(Emoji),
    DeleteEntryButton(Emoji),
    ResetEntryButtons,
}


impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx:&Context<Self>) -> Self {
        let nav_to = Some(ctx.link().callback(|page| Msg::NavigateTo(page)));
        
        let add_entry = Some(ctx.link().callback(|entry| Msg::AddEntry(entry)));

        let add_entry_button = Some(ctx.link().callback(|emoji| Msg::AddEntryButton(emoji)));
        let del_entry_button = Some(ctx.link().callback(|emoji| Msg::DeleteEntryButton(emoji)));
        let reset_entry_buttons = Some(ctx.link().callback(|_| Msg::ResetEntryButtons));

        let gratitude_list_repo = GratitudeListRepo::new();
        let gratitude_list = gratitude_list_repo.read();

        let entry_buttons_repo = EntryButtonsRepo::new();
        let entry_buttons = entry_buttons_repo.read();

        Self {
            page: Page::Daily,
            gratitude_list_repo,
            gratitude_list,
            entry_buttons_repo,
            entry_buttons,
            nav_to,
            add_entry,
            add_entry_button,
            del_entry_button,
            reset_entry_buttons,
        }
    }

    fn update(&mut self, _:&Context<Self>,msg: Self::Message) -> bool {
        match msg {
            Msg::AddEntry(entry) => {
                self.gratitude_list.add(entry);
                self.gratitude_list_repo.save(&self.gratitude_list)
            }
            Msg::NavigateTo(page) => self.page = page,
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

//TODO is it ok to gag this?
    /* 
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
*/
    fn view(&self,_:&Context<Self>) -> Html {
        html! {
            <>
            { self.view_nav() }
            <main>{
                match self.page {
                    Page::Daily => self.view_daily(),
                    Page::History => self.view_history(),
                    Page::Config => self.view_config(),
                }
            }</main>
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
        html! {
            <Nav
                page={self.page}
                nav_to={self.nav_to.as_ref().expect("nav cb")}
            />
        }

    }
}
