use crate::components::layout::footer::Footer;
use crate::components::layout::header::Header;
use crate::pages::about::About;
use crate::pages::events::Events;
use crate::pages::index::Index;
use crate::pages::resources::Resources;
use crate::router::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

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
            <Router<AppRoute, ()>
            render = Router::render(|switch: AppRoute| Self::switch(switch))
        />
            <Footer />
          </div>
        }
    }
}

impl App {
    fn switch(switch: AppRoute) -> Html {
        match switch {
            AppRoute::Resources => html! { <Resources /> },
            AppRoute::Events => html! { <Events /> },
            AppRoute::About => html! { <About /> },
            AppRoute::Index => {
                html! { <Index /> }
            }
        }
    }
}
