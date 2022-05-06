use super::Page;
use yew::prelude::*;
use yew::Context;

pub struct Nav {
    pub link: ComponentLink<Self>,
    pub props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub page: Page,
    pub nav_to: Callback<Page>,
}

#[derive(Copy, Clone)]
pub struct NavMsg(Page);

impl Component for Nav {
    type Message = NavMsg;
    type Properties = Props;
    fn create( ctx: &Context<Self>) -> Self {
        Self { link: ctx.link(), props: ctx.props().clone() }
    }
    fn update(&mut self,_:&Context<Self>, msg: Self::Message) -> bool {
        self.props.nav_to.emit(msg.0);
        false
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
        
        html! {
            <div id="nav">
                { self.span_element("Daily", Page::Daily) } 
                { " | " } 
                { self.span_element("History", Page::History) } 
                { " | " } 
                { self.span_element("Config", Page::Config) }
            </div>
        }
    }

}

impl Nav {

    fn span_element(&self, text: &str, target: Page) -> Html {
        if self.props.page == target {
            html!{ <span><strong>{ text }</strong></span>}
        } else {
            html!{ 
                <span
                    onclick = {self.link.callback(move|_| NavMsg(target))}>
                    { text }
                </span>
            }
        }
    }
}