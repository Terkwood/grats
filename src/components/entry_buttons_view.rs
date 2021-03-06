use crate::model::*;
use yew::prelude::*;

pub struct EntryButtonsView {
    pub link: ComponentLink<Self>,
    pub props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub entry_buttons: EntryButtonCollection,
    pub add_entry_button: Callback<Emoji>,
    pub del_entry_button: Callback<Emoji>,
    pub reset_entry_buttons: Callback<()>,
}

pub enum Msg {
    AddButton(Emoji),
    DelButton(Emoji),
    Reset,
}

impl Component for EntryButtonsView {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddButton(emoji) => {
                self.props.add_entry_button.emit(emoji);
                true
            }
            Msg::DelButton(emoji) => {
                self.props.del_entry_button.emit(emoji);
                true
            }
            Msg::Reset => {
                self.props.reset_entry_buttons.emit(());
                true
            }
        }
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
        let free = self.props.entry_buttons.free_user_buttons();
        html! {
            <div class="configsection">
                { self.view_intro() }
                <div> {
                    if free > 0 {
                        format!("You may add {} more button{}.", free, if free == 1 { "" } else { "s" })
                    } else {
                        "You need to delete a button before you can add another.".to_string()
                    }
                } </div>
                {
                    if free > 0 {
                        self.view_emoji_selection_buttons()
                    } else {
                        html! { <></> }
                    }
                }
                { self.view_reset() }
            </div>
        }
    }
}

impl EntryButtonsView {
    fn view_emoji_selection_buttons(&self) -> Html {
        self.props.entry_buttons.allowed_emojis()
            .iter()
            .map(|emoji|{ let e = emoji.clone();
            html! {
                <button class="big" onclick={self.link.callback(move |_| Msg::AddButton(Emoji(e.clone())))}>{ emoji }</button>
            }})
            .collect::<Html>()
    }

    fn view_intro(&self) -> Html {
        html! {
            <>
                <h1>{ "Configure Entry Buttons"}</h1>
                <h2>{ "Current buttons:" }</h2>
                <ul> { self.props.entry_buttons.all().iter().map(|it| self.view_current_entry_button(it)).collect::<Html>() } </ul>
                <h2>{ "Add a button" }</h2>
            </>
        }
    }

    fn view_current_entry_button(&self, emoji: &Emoji) -> Html {
        let emc = emoji.clone();
        html! {
            <li class="big">
                { emoji.0.clone() }
                {
                    html! {
                        <button
                            class="entry_button"
                            onclick={self.link.callback(move |_| Msg::DelButton(emc.clone()))}>
                            { "DELETE 🗑" }
                        </button>
                    }
                }
            </li>
        }
    }

    fn view_reset(&self) -> Html {
        html! {
            <>
                <h2>{ "Reset Default Buttons"}</h2>
                <div>{ "You may reset the app to use the default buttons." }</div>
                <button
                    class="entry_button"
                    onclick={self.link.callback(move |_| Msg::Reset)}>
                    { "RESET 🔁" }
                </button>
            </>
        }
    }
}
