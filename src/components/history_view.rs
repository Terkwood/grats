use crate::model::*;
use crate::time::*;
use chrono::prelude::*;
use yew::prelude::*;
use yew::Context;
pub struct HistoryView {
    history: History,
    props: Props,
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

    fn create(ctx: &Context<Self>) -> Self {
        let history = History::from(&ctx.props().gratitude_list, js_local_offset());

        Self {
            history,
            props: ctx.props().clone(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {}
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if &self.props != ctx.props() {
            self.props = ctx.props().clone();
            true
        } else {
            false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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
                <ul class="gratitude">{ day.gratitude_list.entries.iter().map(|entry| self.view_entry(entry)).collect::<Html>() }</ul>
            </div>
        }
    }

    fn view_entry(&self, entry: &Entry) -> Html {
        let local_datetime = Utc
            .timestamp_millis(entry.time.0 as i64)
            .with_timezone(&js_local_offset());
        let date_string: String = local_datetime.format("%R").to_string();

        html! {
            <li>
                <span>{
                    format!("{} {} " , entry.emoji.0,  entry.text)
                }</span>
                <span class="history_time">{
                    format!("({})", date_string)
                }</span>
            </li>
        }
    }
}
