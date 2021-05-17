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
            <section class="container flex flex-wrap px-5 py-24 mx-auto items-center">
                <div class="md:w-1/2 md:pr-12 md:py-8 md:border-r md:border-b-0 mb-10 md:mb-0 pb-10 border-b border-gray-200 inline-flex items-end flex-col">
                    <div class="flex flex-col items-center">
                        <img src="fa-brands_meetup.svg" />
                        {"¡Nuestro grupo de meetup!"}
                    </div>
                </div>
                <div class="md:w-1/2 md:pl-12 md:py-8 mb-10 md:mb-0 pb-10 inline-flex items-start flex-col">
                    <div class="flex flex-col items-center">
                        <img src="fa-brands_telegram.svg" />
                        {"¡Nuestro grupo de Telegram!"}
                    </div>
                </div>
            </section>
        }
    }
}
