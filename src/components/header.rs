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
            <header class="bg-white font-header h-23 text-gray-600 body-font filter shadow-md drop-shadow-2xl">
                <div class="container mx-auto flex flex-wrap p-5 flex-col md:flex-row items-center">
                <a class="flex h-16 items-center text-gray-900">
                    <img class="text-white rounded-full h-32 w-32 top-2" src="https://rustacean.net/assets/rustacean-flat-happy.svg" />
                    <h2 class="ml-3 min-w-full text-4xl font-playfair">{"Rust Argentina"}</h2>
                </a>
                <nav class="md:ml-auto text-xl flex flex-wrap items-center text-base justify-center font-lato">
                <a class="mr-7 hover:text-gray-900">{"Acerca"}</a>
                <a class="mr-7 hover:text-gray-900">{"Eventos"}</a>
                <a class="mr-7 hover:text-gray-900">{"Guía de Recursos"}</a>
              </nav>
                </div>
            </header>
        }
    }
}
