use crate::router::AppRoute;
use yew::prelude::*;
use yew_router::components::RouterAnchor;

pub struct About {}

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _: ComponentLink<Self>) -> Self {
        About {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <section class="about-container">
                <div class="about-left-side">
                    <img class="about-image" src="https://i.imgur.com/tsAmqTh.jpg" />
                </div>
                <div class="about-right-side">
                    <p class="about-description">
                        {"Desde 2018 hemos organizado eventos en la Ciudad Autónoma de Buenos Aires."}
                        <br/>
                        {"Dando clases en Facultades y acercando oradores del exterior para dar charlas y conocer a la comunidad."}
                    </p>
                    <RouterAnchor<AppRoute> classes="about-action" route=AppRoute::About>
                                {"Acerca"}
                    </RouterAnchor<AppRoute>>
                </div>
            </section>
        }
    }
}
