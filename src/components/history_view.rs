use crate::model::*;
use yew::prelude::*;
struct HistoryView {
    history: History,
    props: Props,
    link: ComponentLink<Self>,
}
#[derive(Properties, Clone, PartialEq)]
struct Props {
    gratitude_list: GratitudeList,
}
enum Msg {}

const EMPTY_MSG: &str =
    "This is your history of daily gratitude entries.  You haven't written anything down yet.";

impl Component for HistoryView {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let history = History::from(&props.gratitude_list, todo!("offset"));

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
        todo!()
    }
}
