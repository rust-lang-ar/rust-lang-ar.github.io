use crate::components::index::hero::Hero;
use crate::components::index::joinus::JoinUs;
use crate::components::index::our_members::OutMembers;
use crate::components::index::our_projects::OurProjects;
use crate::components::layout::footer::Footer;
use crate::components::layout::header::Header;

use yew::prelude::*;

pub struct Events {}

impl Component for Events {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Events {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <section class="our-projects-section">
            <div class="our-projects-title-container">
                <span class="our-projects-title">{"PROXIMOS EVENTOS"}</span>
            </div>
          </section>
        }
    }
}
