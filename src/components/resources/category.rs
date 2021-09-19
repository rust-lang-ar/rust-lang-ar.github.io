use yew::prelude::*;

pub struct Category {
    link: ComponentLink<Self>,
    value: bool,
    props: ListProps,
}

#[derive(Properties, Clone)]
pub struct ListProps {
    pub title: String,
    #[prop_or_default]
    pub children: Children,
}

pub enum Msg {
    Toogle,
}

impl Component for Category {
    type Message = Msg;
    type Properties = ListProps;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: false,
            props: _props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        self.value = !self.value;
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        fn render_if_true(value: &bool, props: &ListProps) -> Html {
            if *value {
                html! {
                    <div class="list">
                        { for props.children.iter() }
                    </div>
                }
            } else {
                html! {}
            }
        }

        html! {
            <div class="my-10">
              <button class="category-button" onclick=self.link.callback(|_| Msg::Toogle)>
              <p>{&self.props.title}</p>
              { if self.value {
                html! {
                    <img src="images/arrow-up-solid.svg" alt="Ocultar recursos" />
                }}else{
                    html! {
                        <img src="images/arrow-down-solid.svg" alt="Mostrar recursos" />
                    }}
              }
              </button>
              { render_if_true(&self.value, &self.props) }
            </div>
        }
    }
}
