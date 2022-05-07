use super::Page;
use yew::prelude::*;
use yew::Context;

// TODO: remove props ?!
pub struct Nav;

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
        Self { }
    }
    fn update(&mut self,ctx:&Context<Self>, msg: Self::Message) -> bool {
        ctx.props().nav_to.emit(msg.0);
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        
        html! {
            <div id="nav">
                { self.span_element("Daily", Page::Daily, ctx) } 
                { " | " } 
                { self.span_element("History", Page::History, ctx) } 
                { " | " } 
                { self.span_element("Config", Page::Config, ctx) }
            </div>
        }
    }

}

impl Nav {

    fn span_element(&self, text: &str, target: Page, ctx: &Context<Self>) -> Html {
        if ctx.props().page == target {
            html!{ <span><strong>{ text }</strong></span>}
        } else {
            html!{ 
                <span
                    onclick = {ctx.link().callback(move|_| NavMsg(target))}>
                    { text }
                </span>
            }
        }
    }
}