use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct Hero {}

pub enum Msg {}

impl Component for Hero {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Hero { }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <main class="container px-5 py-48 mx-aut lg:w-4/5 mx-auto flex flex-wrap  items-center">
                <div class="lg:w-1/2 w-full lg:pl-10 lg:py-6 mt-6 lg:mt-0">
                    <h1 class="text-gray-900 text-6xl title-font font-lato mb-1">
                    {"LA COMUNIDAD DE RUST EN ARGENTINA ESTÁ CRECIENDO"}
                    </h1>
                    <h2 class="text-2xl font-playfair text-gray-500 tracking-widest">
                    {"Si te interesa aprender Rust o estás llevando a cabo un proyecto con tecnología asociada, te invitamos a acercarte y participar."}
                    </h2>
                </div>
                <img src="https://foundation.rust-lang.org/img/four-oh-fourris.png" class="lg:w-1/2 w-full lg:h-auto h-64 object-cover object-center rounded" />
            </main>
        }
    }
}
