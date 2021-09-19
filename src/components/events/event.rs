use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Event {
    pub title: String,
    pub description: String,
    pub date: String,
}

impl Component for Event {
    type Message = ();
    type Properties = Event;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            title: props.title,
            description: props.description,
            date: props.date,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="events-container">
                <div class="events-left-side">
                    <h2 class="event-title">{&self.title}</h2>
                    <p class="event-text">{&self.description}</p>
                </div>
                <div class="events-right-side">
                    <p class="event-text">{&self.date}</p>
                    <h2 class="event-sign-up">{"Inscribirse"}</h2>
                </div>
            </div>
        }
    }
}
