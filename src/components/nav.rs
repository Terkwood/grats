use super::Page;
use yew::prelude::*;

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
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.props.nav_to.emit(msg.0);
        false
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
        let page = self.props.page;
        let daily_nav = if page == Page::Daily {
            html!{ <span class="nav"><strong>{ "Daily" }</strong></span>}
        } else {
            html!{ 
                <span class="nav" 
                    onclick = self.link.callback(move|_| NavMsg(Page::Daily))>
                    { "Daily" }
                </span>
            }
        };
        let history_nav = if page == Page::History {
            html!{ <span class="nav">{ "History" }</span>}
        } else {
            html!{ 
                <span class="nav" 
                    onclick = self.link.callback(move|_| NavMsg(Page::History))>
                    { "History" }
                </span>
            }
        };
        let config_nav = if page == Page::Config {
            html!{ <span class="nav">{ "Config" }</span>}
        } else {
            html!{ 
                <span class="nav" 
                    onclick = self.link.callback(move|_| NavMsg(Page::Config))>
                    { "Config" }
                </span>
            }
        };
        
        html! {
            <div>
                { daily_nav } 
                { "|" } 
                { history_nav } 
                { "|" } 
                { config_nav }
            </div>
        }
    }
}
