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
    pub resolve_item: Callback<UtcMillis>,
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

impl Daily {
    pub fn view_input(&self) -> Html {
        let input_grid_id = if self.mode == Mode::Input {
            "inputgridfocus"
        } else {
            "inputgridwaiting"
        };
        html! {
            <div id=input_grid_id>
                <div id="bigtextgrid">
                    <textarea
                        value=&self.text_area
                        onfocus=self.link.callback(|_| Msg::FocusInput)
                        oninput=self.link.callback(|e: InputData| Msg::TextAreaUpdated(e.value))
                        placeholder="What are you grateful for?">
                    </textarea>
                </div>
                { self.props.entry_buttons.all().iter().map(|emoji| self.view_entry_button(emoji)).collect::<Html>()}
            </div>
        }
    }
    fn view_todays_list(&self) -> Html {
        todo!()
    }
    fn view_entry_button(&self, emoji: &Emoji) -> Html {
        todo!()
    }
}
