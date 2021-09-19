use crate::components::layout::header::Header;
use crate::components::layout::footer::Footer;
use crate::pages::index::Index;
use crate::pages::about::About;
use crate::pages::events::Events;

use yew::prelude::*;
use crate::router::{AppAnchor, AppRoute, AppRouter, PublicUrlSwitch};

pub struct App {
    link: ComponentLink<Self>,
    navbar_active: bool,
}

pub enum Msg {
    ToggleNavbar,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            navbar_active: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
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
            AppRoute::Events => 
                html! { <Events /> },
            AppRoute::About => 
                html! { <About /> },
            AppRoute::Index => {
                html! { <Index /> }
            }
        }
    }
}
