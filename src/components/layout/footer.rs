use crate::router::AppRoute;
use yew::prelude::*;
use yew_router::components::RouterAnchor;

pub struct Footer {}

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <footer class="text-gray-600 body-font">
            <div class="container px-5 py-2 mx-auto">
            <div class="flex flex-wrap md:text-left text-center order-first justify-between">

                <div class="lg:w-1/4 md:w-1/2 w-full px-4">
                    <nav class="list-none">
                        <li class="my-2">
                            <RouterAnchor<AppRoute> classes="font-heading text-gray-600 hover:text-gray-800" route=AppRoute::About>
                                {"Acerca"}
                            </RouterAnchor<AppRoute>>
                        </li>
                        <li class="my-2">
                            <RouterAnchor<AppRoute> classes="font-heading text-gray-600 hover:text-gray-800" route=AppRoute::Events>
                                {"Eventos"}
                            </RouterAnchor<AppRoute>>
                        </li>
                        <li class="my-2">
                            <RouterAnchor<AppRoute> classes="font-heading text-gray-600 hover:text-gray-800" route=AppRoute::Resources>
                                {"Guia de Recursos"}
                            </RouterAnchor<AppRoute>>
                        </li>
                    </nav>
                </div>
                <RouterAnchor<AppRoute> classes="lg:w-1/4 md:w-1/2 w-full px-4 flex items-center gap-3 justify-center" route=AppRoute::Index>
                    <img class="header-logo" src="images/rust-lang-ar-logo.png" />
                    <p class="font-heading text-xl text-gray-600 hover:text-gray-800">
                        {"Rust"}
                    <br class="lg:block hidden" />
                        {"Argentina"}
                    </p>
                </RouterAnchor<AppRoute>>
            </div>
          </div>
          <hr class="border-b border-gray-100" />
            <div class="container px-5 py-8 mx-auto flex items-center sm:flex-row flex-col">
            <p class="text-gray-500 text-sm">
            {"Copyright 2021"}
            </p>
            </div>

            </footer>
        }
    }
}
