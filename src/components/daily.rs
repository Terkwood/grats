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

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        todo!()
    }
}
