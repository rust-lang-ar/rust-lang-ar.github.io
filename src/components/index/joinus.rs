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
                            <img src="fa-brands_meetup.svg" />
                        </div>
                        {"¡Nuestro grupo de meetup!"}
                    </div>
                </div>
                <div class="joinus-right-side">
                    <div class="joinus-side-container">
                        <div class="icon-container">
                            <img src="fa-brands_telegram.svg" />
                        </div>
                        {"¡Nuestro grupo de Telegram!"}
                    </div>
                </div>
            </section>
        }
    }
}
