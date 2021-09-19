use yew::prelude::*;
use crate::router::{AppAnchor, AppRoute, AppRouter, PublicUrlSwitch};

pub struct Header {}

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Header {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <header class=classes!("header")>
                <div class=classes!("header-container")>
                    <AppAnchor classes="header-left-side" route=AppRoute::Index>
                        <img class="header-logo" src="images/rust-lang-ar-logo.png" />
                        <h2 class="header-title">{"Rust Argentina"}</h2>
                    </AppAnchor>
                    <nav class="header-right-side">
                        <AppAnchor classes="header-nav-item" route=AppRoute::About>
                            {"Acerca"}
                        </AppAnchor>
                        <a class="header-nav-item">{"Eventos"}</a>
                        <a class="header-nav-item">{"Guía de Recursos"}</a>
                    </nav>
                </div>
            </header>
        }
    }
}
