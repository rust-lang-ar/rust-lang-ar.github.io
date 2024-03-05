use crate::components::index::about::About;
use crate::components::index::hero::Hero;
use crate::components::index::joinus::JoinUs;
use crate::components::index::our_members::OutMembers;
use crate::components::index::our_projects::OurProjects;

use yew::prelude::*;

pub struct Index {}

impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Index {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <>
            <Hero />
            <JoinUs />
            <About />
            <OurProjects />
            <OutMembers />
          </>
        }
    }
}
