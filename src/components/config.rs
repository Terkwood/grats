use super::EntryButtonsView;
use crate::model::*;
use crate::time::js_utc_now;
use yew::prelude::*;
use yew_export_button::{export_button, ButtonOpts};

pub struct Config {
    pub link: ComponentLink<Self>,
    pub props: Props,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub entry_buttons: EntryButtonCollection,
    pub gratitude_list: GratitudeList,
    pub add_entry_button: Callback<Emoji>,
    pub del_entry_button: Callback<Emoji>,
    pub reset_entry_buttons: Callback<()>,
}

impl Component for Config {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }
    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
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
        html! {
            <div>
                { self.view_export() }
                { self.view_inventory_buttons() }
                { self.view_about() }
            </div>
        }
    }
}

const REPO_URL: &str = "https://github.com/Terkwood/grats";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const EXPORT_FILE_PREFIX: &str = "grats";
const EXPORT_BUTTON_CSS_ID: &str = "export_button";
const EXPORT_A_CSS_CLASS: &str = "download";
impl Config {
    fn view_export(&self) -> Html {
        let button = export_button(
            &self.props.gratitude_list,
            ButtonOpts {
                utc_millis: js_utc_now().0,
                a_class: EXPORT_A_CSS_CLASS,
                button_id: EXPORT_BUTTON_CSS_ID,
                file_prefix: EXPORT_FILE_PREFIX,
            },
        );
        html! {
            <div class="configsection">
                <h1>{ "Export Data"}</h1>
                { button }
            </div>
        }
    }

    fn view_inventory_buttons(&self) -> Html {
        html! {
            <EntryButtonsView
                entry_buttons=self.props.entry_buttons.clone()
                add_entry_button=self.props.add_entry_button.clone()
                del_entry_button=self.props.del_entry_button.clone()
                reset_entry_buttons=self.props.reset_entry_buttons.clone()
            />
        }
    }

    fn view_about(&self) -> Html {
        html! {
            <div class="configsection">
                <h1>{ "About" }</h1>
                <p>{ "GRATS helps you make a list of things which bring you joy." }</p>
                <p>{ "GRATS is designed with privacy in mind.  Your data will never be transmitted to a third party.  Data is kept in browser local storage, unencypted.  KEEP YOUR DATA SAFE: make sure there is no malware on your system!" }</p>
                <p>{ format!("This is version {}.", VERSION) }</p>
                <h2>{ "Source Code" }</h2>
                <p>{ "The source code is available under MIT license." }</p>
                <p><a href=REPO_URL>{ REPO_URL }</a></p>
            </div>
        }
    }
}
