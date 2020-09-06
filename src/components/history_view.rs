use yew::prelude::*;
struct HistoryView {
    props: Props,
    link: ComponentLink<Self>,
}
#[derive(Properties, Clone, PartialEq)]
struct Props {}
enum Msg {}

impl Component for HistoryView {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
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
        html! {}
    }
}
