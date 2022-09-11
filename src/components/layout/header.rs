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
            <header class=classes!("h-20", "bg-red-200")>
                
                // <div class=classes!("header-container")>
                //     <RouterAnchor<AppRoute> classes="header-left-side" route=AppRoute::Index >
                //         <img class="header-logo" src="images/rust-lang-ar-logo.png" />
                //         <h2 class="header-title">{"Rust Argentina"}</h2>
                //     </RouterAnchor<AppRoute>>
                //     <nav class="header-right-side">
                //         <RouterAnchor<AppRoute> classes="header-nav-item" route=AppRoute::About>
                //             {"Acerca"}
                //         </RouterAnchor<AppRoute>>
                //         <RouterAnchor<AppRoute> classes="header-nav-item" route=AppRoute::Events>
                //             {"Eventos"}
                //         </RouterAnchor<AppRoute>>
                //         <RouterAnchor<AppRoute> classes="header-nav-item" route=AppRoute::Resources>
                //             {"Guía de Recursos"}
                //         </RouterAnchor<AppRoute>>
                //     </nav>
                // </div>
            </header>
        }
    }
}
