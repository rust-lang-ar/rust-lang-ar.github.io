use crate::components::events::event::Event;

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
          <section class="events-section">
            <div class="events-title-container">
                <span class="events-title">{"PROXIMOS EVENTOS"}</span>
            </div>
            <div class="events-container-list">
                <hr class="w-full border-b-2 border-gray-200"/>
                <Event title="Meet and Greet Friends from Abroad" description="We heard there are rustaceans from abroad in town so we’re hosting a meet and greet at the LambdaClass offices." date="7 de febrero de 2020 16:30 / 21:30"  />
                <hr class="w-full border-b-2 border-gray-200"/>
                <Event title="WebAssembly y Toy projects en Rust" description="Angel Java Lopez presentara sobre “WebAssembly y Rust”: WebAssembly, formato binario, formato de texto, historia y motivación, máquina de pila, ejecución en el browser y en otros entornos, herramientas de compilación. Rust y WebAssembly: target de compilacion; ejemplos. Federico Carrone hablara sobre como aprender Rust implementando toy projects." date="22 de agosto de 2019 16:30 / 21:30" />
                <hr class="w-full border-b-2 border-gray-200"/>
                </div>
          </section>
        }
    }
}
