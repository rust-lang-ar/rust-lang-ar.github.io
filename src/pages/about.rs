use crate::components::index::hero::Hero;
use crate::components::index::joinus::JoinUs;
use crate::components::index::our_projects::OurProjects;
use crate::components::index::our_members::OutMembers;
use crate::components::layout::header::Header;
use crate::components::layout::footer::Footer;

use yew::prelude::*;

pub struct About {}

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        About {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <section class="about-container">
                <div class="about-left-side">
                    <img class="about-image" src="https://i.imgur.com/tsAmqTh.jpg" />
                </div>
                <div class="about-right-side">
                    <p class="about-description">
                        {"La comunidad de Rust en Argentina es pequeña pero creciente."}
                        <br />
                        <br />
                        {"Desde 2018 hemos organizado meetups de Rust en la Ciudad Autónoma de Buenos Aires, dado clases por invitación en facultades publicas y gestionado acercar oradores del exterior para dar charlas y conocer a la comunidad."}
                        <br />
                        <br />
                        {"Si estas interesado o interesada en aprender Rust, o estas llevando a cabo un proyecto con tecnología asociada, te invitamos a acercarte y participar. "}
                        <br />
                        <br />
                        {"El sitio en Meetup punto com de la comunidad es: Rust-Argentina También tenemos un un canal en Telegram. Agradecemos a LambdaClass por su apoyo a la comunidad."}
                    </p>
                </div>
            </section>
        }
    }
}
