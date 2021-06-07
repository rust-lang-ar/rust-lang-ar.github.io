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
            <main class="hero-container">
                <div class="hero-left-side">
                    <h1 class="hero-title">
                    {"LA COMUNIDAD DE RUST EN ARGENTINA ESTÁ CRECIENDO"}
                    </h1>
                    <h2 class="hero-subtitle">
                    {"Si te interesa aprender Rust o estás llevando a cabo un proyecto con tecnología asociada, te invitamos a acercarte y participar."}
                    </h2>
                </div>
                <img src="Rust_Arg_web.svg" class="hero-right-side" />
            </main>
        }
    }
}
