use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct JoinUS {}

pub enum Msg {}

impl Component for JoinUS {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        JoinUS { }
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
                        <img src="fa-brands_meetup.svg" />
                        {"¡Nuestro grupo de meetup!"}
                    </div>
                </div>
                <div class="joinus-right-side">
                    <div class="joinus-side-container">
                        <img src="fa-brands_telegram.svg" />
                        {"¡Nuestro grupo de Telegram!"}
                    </div>
                </div>
            </section>
        }
    }
}
