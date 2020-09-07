use crate::model::*;
use crate::time::*;
use chrono::prelude::*;
use yew::prelude::*;
pub struct HistoryView {
    history: History,
    props: Props,
    link: ComponentLink<Self>,
}
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub gratitude_list: GratitudeList,
}
pub enum Msg {}

const EMPTY_MSG: &str =
    "This is your history of daily gratitude entries.  You haven't written anything down yet.";

impl Component for HistoryView {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let history = History::from(&props.gratitude_list, js_local_offset());

        Self {
            history,
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {}
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
        let payload = &self.history.days;
        if payload.is_empty() {
            html! { <div id="history"> <p> { EMPTY_MSG } </p> </div> }
        } else {
            html! {
                <div id="history">
                {  payload.iter()
                    .map(|day| self.view_day(day))
                    .collect::<Html>()
                }
                </div>
            }
        }
    }
}

impl HistoryView {
    fn view_day(&self, day: &history::Day) -> Html {
        html! {
            <div>
                <h1>{ day.date.format("%A, %b %e, %Y") }</h1>
                <ul>{ day.gratitude_list.entries.iter().map(|entry| self.view_entry(entry)).collect::<Html>() }</ul>
            </div>
        }
    }

    fn view_entry(&self, entry: &Entry) -> Html {
        let local_datetime = Utc
            .timestamp_millis(entry.time.0 as i64)
            .with_timezone(&js_local_offset());
        let date_string: String = local_datetime.format("%m/%d %R").to_string();

        html! {
            <li class="gratitude_list_entry">
                { format!("{} [{}] {} " , entry.emoji.0, date_string, entry.text) }
            </li>
        }
    }
}
