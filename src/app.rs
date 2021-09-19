use crate::components::layout::footer::Footer;
use crate::components::layout::header::Header;
use crate::pages::about::About;
use crate::pages::events::Events;
use crate::pages::index::Index;
use crate::pages::resources::Resources;

use crate::router::{AppRoute, AppRouter, PublicUrlSwitch};
use yew::prelude::*;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <div >
            <Header />
                <AppRouter
                render=AppRouter::render(Self::switch)
                />
            <Footer />
          </div>
        }
    }
}

impl App {
    fn switch(switch: PublicUrlSwitch) -> Html {
        match switch.route() {
            AppRoute::Resources => html! { <Resources /> },
            AppRoute::Events => html! { <Events /> },
            AppRoute::About => html! { <About /> },
            AppRoute::Index => {
                html! { <Index /> }
            }
        }
    }
}
