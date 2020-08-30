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
    SubmitItem(Emoji),
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
        todo!()
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        todo!()
    }
}
