
use yew::prelude::*;

pub struct Resources {}

impl Component for Resources {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Resources {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <section class="resources-section">
            <div class="resources-title-container">
                <span class="resources-title">{"GUÍA DE RECURSOS"}</span>
                <p>{"Aqui reunimos algunos recursos didácticos para aprender Rust."}</p>
            </div>
            <div class="resources-container-list">
                <hr class="w-full border-b-2 border-gray-200"/>
                <hr class="w-full border-b-2 border-gray-200"/>
                </div>
          </section>
        }
    }
}
