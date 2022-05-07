use crate::model::*;
use crate::time::js_utc_now;
use web_sys::HtmlTextAreaElement;
use yew::events::InputEvent;
use yew::prelude::*;
use yew::Context;

pub struct Daily {
    text_area: String,
    mode: Mode,
}

pub enum Msg {
    SubmitEntry(Emoji),
    TextAreaUpdated(String),
    FocusInput,
    NothingHappened,
}

#[derive(PartialEq)]
pub enum Mode {
    Default,
    Input,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub gratitude_list: GratitudeList,
    pub entry_buttons: EntryButtonCollection,
    pub add_entry: Callback<Entry>,
}

impl Component for Daily {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            text_area: String::new(),
            mode: Mode::Default,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FocusInput => {
                self.mode = Mode::Input;
            }
            Msg::TextAreaUpdated(text) => self.text_area = text,
            Msg::SubmitEntry(emoji) => {
                self.mode = Mode::Default;
                if !self.text_area.is_empty() {
                    ctx.props().add_entry.emit(Entry {
                        emoji,
                        text: self.text_area.clone(),
                        time: js_utc_now(),
                    });

                    self.text_area.clear()
                };
            }
            _ => (),
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                { self.view_input(ctx) }
                { self.view_todays_list(ctx) }
            </>
        }
    }
}

const GRID_TEMPLATE_ROWS_WAITING: u8 = 6;
const GRID_TEMPLATE_ROWS_FOCUS: u8 = 7;

impl Daily {
    pub fn view_input(&self, ctx: &Context<Self>) -> Html {
        let entry_buttons = ctx.props().entry_buttons.all();
        let (input_grid_id, input_grid_rows) = if self.mode == Mode::Input {
            ("inputgridfocus", GRID_TEMPLATE_ROWS_FOCUS)
        } else {
            ("inputgridwaiting", GRID_TEMPLATE_ROWS_WAITING)
        };
        html! {
                    <div
                        id={input_grid_id}
                        style={format!("grid-template: repeat({}, 1fr) / repeat({}, 1fr);", input_grid_rows, entry_buttons.len())}
                        >
                        <div style={format!("grid-column: 1 / span {}; grid-row: 1 / span {};", entry_buttons.len(), GRID_TEMPLATE_ROWS_WAITING)}>
                            <textarea
                                value={self.text_area}
                                onfocus={ctx.link().callback(|_| Msg::FocusInput)}
                                oninput={ctx.link().callback(|e: InputEvent|
                                    if let Some(input) = e.target_dyn_into::<HtmlTextAreaElement>() {
                                        Msg::TextAreaUpdated(input.value())
                                    }  else {
        Msg::NothingHappened
                                    }
                                )}
                                placeholder="What are you grateful for?">
                            </textarea>
                        </div>
                        { entry_buttons.iter().map(|emoji| self.view_entry_button(emoji,ctx)).collect::<Html>()}
                    </div>
                }
    }
    fn view_todays_list(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="center">
                <ul class="gratitude">
                    { ctx.props().gratitude_list.entries.iter().map(|entry| self.view_entry(entry.clone())).collect::<Html>() }
                </ul>
            </div>
        }
    }
    fn view_entry(&self, entry: Entry) -> Html {
        html! {
            <li class="entry">
                { format!("{} {} " , entry.emoji.0, entry.text) }
            </li>
        }
    }
    fn view_entry_button(&self, emoji: &Emoji, ctx: &Context<Self>) -> Html {
        let emc = emoji.clone();
        let emc2 = emoji.clone();
        html! {
            <div class="center">
                    <button
                        class="entrybutton"
                        onclick=
         { ctx.link()
                                .callback(
                                    move |_| Msg::SubmitEntry(
                                        emc.clone()
                                    ))}>
                        { emc2.0 }
                    </button>
                </div>
        }
    }
}
