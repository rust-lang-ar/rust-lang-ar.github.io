use crate::components::index::about::About;
use crate::components::index::hero::Hero;
use crate::components::index::joinus::JoinUs;
use crate::components::index::our_projects::OurProjects;
use crate::components::layout::header::Header;

use yew::prelude::*;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <div >
            <Header />
            <Hero />
            <JoinUs />
            <About />
            <OurProjects />
          </div>
        }
    }
}
