use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct Header {}

pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Header { }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <header class="bg-header font-header h-11 text-gray-600 body-font filter shadow-md drop-shadow-2xl">
                <div class="container mx-auto flex flex-wrap flex-col md:flex-row items-center">
                <a class="flex h-11 title-font font-medium items-center text-gray-900">
                    <img class="w-14 h-14 text-white rounded-full relative top-2 shadow-md" src="https://raw.githubusercontent.com/rust-lang-ar/rust-lang-ar.github.io/development/src/img/rust-lang-ar-logo.png" />
                    <h2 class="ml-3 text-xl font-bold">{"Rust Argentina"}</h2>
                </a>
                <nav class="md:ml-auto h-11 flex flex-wrap items-center text-base justify-center font-medium">
                <a class="mr-7 hover:text-gray-900">{"Acerca"}</a>
                <a class="mr-7 hover:text-gray-900">{"Eventos"}</a>
                <a class="mr-7 hover:text-gray-900">{"Guía de Recursos"}</a>
              </nav>
                </div>
            </header>
        }
    }
}
