use yew::prelude::*;

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <div>
            <figure class="bg-gray-100 rounded-xl p-8">
              <img class="w-32 h-32 rounded-full" src="https://avatars.githubusercontent.com/u/19656993?v=4" alt="" width="384" height="512" />
              <div class="pt-6 space-y-4">
                <blockquote>
                  <p class="text-lg">
                    {r#"Student in Computer Science degree at Universidad Nacional del Oeste"#}
                  </p>
                </blockquote>
                <figcaption>
                  <div>
                    {"Fernando Pastorelli"}
                  </div>
                  <div>
                    {"Developer Consultant at gA"}
                  </div>
                </figcaption>
              </div>
            </figure>
            <figure class="bg-gray-100 rounded-xl p-8">
              <img class="w-32 h-32 rounded-full" src="https://avatars.githubusercontent.com/u/34756077?v=4" alt="" width="384" height="512" />
              <div class="pt-6 space-y-4">
                <blockquote>
                  <p class="text-lg">
                    {r#"Software Engineer. I enjoy writing software with @rust-lang and TypeScript"#}
                  </p>
                </blockquote>
                <figcaption>
                  <div>
                    {"Esteban Borai"}
                  </div>
                  <div>
                    {"Software Engineer"}
                  </div>
                </figcaption>
              </div>
            </figure>
          </div>
        }
    }
}
