use yew::prelude::*;

pub struct JoinUs {}

impl Component for JoinUs {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _: ComponentLink<Self>) -> Self {
        JoinUs {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <section class="joinus-container">
                <div class="joinus-left-side">
                    <div class="joinus-side-container">
                        <div class="icon-container">
                        <a href="https://www.meetup.com/es-ES/Rust-Argentina/" target="_blank">
                        <img src="images/fa-brands_meetup.svg" />
                            </a>
                            </div>
                        <a href="https://www.meetup.com/es-ES/Rust-Argentina/" target="_blank">
                        {"¡Nuestro grupo de meetup!"}
                        </a>
                        </div>
                </div>
                <div class="joinus-right-side">
                    <div class="joinus-side-container">
                    <div class="icon-container">
                        <a href="https://t.me/rust_lang_ar" target="_blank">
                            <img src="images/fa-brands_telegram.svg" />
                            </a>
                        </div>
                        <a href="https://t.me/rust_lang_ar" target="_blank">
                        {"¡Nuestro grupo de Telegram!"}
                        </a>
                        </div>
                </div>
            </section>
        }
    }
}
