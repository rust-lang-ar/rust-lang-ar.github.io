use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Header {}

pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Header { }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <header class="header">
                <div class="header-container">
                    <a class="header-left-side">
                        <img class="header-logo" src="rust-lang-ar-logo.png" />
                        <h2 class="header-title">{"Rust Argentina"}</h2>
                    </a>
                    <nav class="header-right-side">
                        <a class="header-nav-item">{"Acerca"}</a>
                        <a class="header-nav-item">{"Eventos"}</a>
                        <a class="header-nav-item">{"Guía de Recursos"}</a>
                    </nav>
                </div>
            </header>
        }
    }
}
