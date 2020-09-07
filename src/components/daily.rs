use crate::model::*;
use crate::time::js_utc_now;
use yew::prelude::*;

pub struct Daily {
    link: ComponentLink<Self>,
    props: Props,
    text_area: String,
    mode: Mode,
}

pub enum Msg {
    SubmitEntry(Emoji),
    TextAreaUpdated(String),
    FocusInput,
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
    pub show_nav: Callback<bool>,
}

impl Component for Daily {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            text_area: String::new(),
            mode: Mode::Default,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FocusInput => {
                self.mode = Mode::Input;
                self.props.show_nav.emit(false)
            }
            Msg::TextAreaUpdated(text) => self.text_area = text,
            Msg::SubmitEntry(emoji) => {
                self.mode = Mode::Default;
                self.props.show_nav.emit(true);
                if !self.text_area.is_empty() {
                    self.props.add_entry.emit(Entry {
                        emoji,
                        text: self.text_area.clone(),
                        time: js_utc_now(),
                    });

                    self.text_area.clear()
                };
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                { self.view_input() }
                { self.view_todays_list() }
            </>
        }
    }
}

const GRID_TEMPLATE_ROWS_WAITING: u8 = 6;
const GRID_TEMPLATE_ROWS_FOCUS: u8 = 7;

impl Daily {
    pub fn view_input(&self) -> Html {
        let entry_buttons = self.props.entry_buttons.all();
        let (input_grid_id, input_grid_rows) = if self.mode == Mode::Input {
            ("inputgridfocus", GRID_TEMPLATE_ROWS_FOCUS)
        } else {
            ("inputgridwaiting", GRID_TEMPLATE_ROWS_WAITING)
        };
        html! {
            <div
                id=input_grid_id
                style=format!("grid-template: repeat({}, 1fr) / repeat({}, 1fr);", input_grid_rows, entry_buttons.len())
                >
                <div style=format!("grid-column: 1 / span {}; grid-row: 1 / span {};", entry_buttons.len(), GRID_TEMPLATE_ROWS_WAITING)>
                    <textarea
                        value=&self.text_area
                        onfocus=self.link.callback(|_| Msg::FocusInput)
                        oninput=self.link.callback(|e: InputData| Msg::TextAreaUpdated(e.value))
                        placeholder="What are you grateful for?">
                    </textarea>
                </div>
                { entry_buttons.iter().map(|emoji| self.view_entry_button(emoji)).collect::<Html>()}
            </div>
        }
    }
    fn view_todays_list(&self) -> Html {
        html! {
            <div class="center">
                <ul class="gratitude">
                    { self.props.gratitude_list.entries.iter().map(|entry| self.view_entry(entry.clone())).collect::<Html>() }
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
    fn view_entry_button(&self, emoji: &Emoji) -> Html {
        let emc = emoji.clone();
        let emc2 = emoji.clone();
        html! {
            <div class="center">
                    <button
                        class="entrybutton"
                        onclick=
                            self.link
                                .callback(
                                    move |_| Msg::SubmitEntry(
                                        emc.clone()
                                    ))>
                        { emc2.0 }
                    </button>
                </div>
        }
    }
}
