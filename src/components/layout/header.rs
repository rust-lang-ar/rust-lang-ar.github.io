use yew::prelude::*;
use yew_router::components::RouterAnchor;

use crate::router::AppRoute;

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
            <header class="shadow h-20 bg-white">
                <div class="flex items-center justify-between w-11/12 mx-auto py-2">
                    <RouterAnchor<AppRoute> classes="flex items-center gap-4" route=AppRoute::Index >
                        <img
                            alt="Rust Language Argentina"
                            class="h-16"
                            src="images/rust-lang-ar-logo.png"
                            height="64"
                            width="64"
                        />
                        <h1>{"Rust Argentina"}</h1>
                    </RouterAnchor<AppRoute>>
                    <nav class="flex gap-4 w-max">
                        <RouterAnchor<AppRoute> classes="header-nav-item" route=AppRoute::About>
                            {"Acerca"}
                        </RouterAnchor<AppRoute>>
                        <RouterAnchor<AppRoute> classes="header-nav-item" route=AppRoute::Events>
                            {"Eventos"}
                        </RouterAnchor<AppRoute>>
                        <RouterAnchor<AppRoute> classes="header-nav-item" route=AppRoute::Resources>
                            {"Guía de Recursos"}
                        </RouterAnchor<AppRoute>>
                    </nav>
                </div>
            </header>
        }
    }
}
