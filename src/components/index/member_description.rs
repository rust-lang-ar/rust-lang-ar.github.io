use anyhow::Error;
use serde::Deserialize;
use yew::format::Nothing;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::ConsoleService;

#[derive(Deserialize, Clone, Debug)]
pub struct User {
    pub name: String,
    pub bio: Option<String>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub user_url: String,
}

pub struct MemberDescription {
    is_fetching: bool,
    fetch_failed: bool,
    fetch_task: Option<FetchTask>,
    has_member: bool,
    link: ComponentLink<Self>,
    member: Option<User>,
    user_url: String,
}

pub enum Msg {
    FetchFailed,
    FetchSuccess(User),
}

impl Component for MemberDescription {
    type Message = Msg;
    type Properties = Props;

    fn create(prop: Props, link: ComponentLink<Self>) -> Self {
        Self {
            is_fetching: false,
            fetch_failed: false,
            fetch_task: None,
            has_member: false,
            link,
            member: None,
            user_url: prop.user_url,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchSuccess(member) => {
                self.has_member = true;
                self.is_fetching = false;
                self.fetch_failed = false;
                self.member = Some(member);

                true
            }
            Msg::FetchFailed => {
                self.is_fetching = false;
                self.fetch_failed = true;

                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn rendered(&mut self, _first_render: bool) {
        if _first_render {
            self.is_fetching = true;

            let callback = self
                .link
                .callback(move |response: Response<Result<String, Error>>| {
                    let (meta, body) = response.into_parts();

                    if meta.status.is_success() {
                        match body {
                            Ok(body) => {
                                ConsoleService::info("Fetched members list");
                                let body = body.as_str();

                                if let Ok(member) = serde_json::from_str::<User>(body) {
                                    return Msg::FetchSuccess(member);
                                }
                            }
                            Err(err) => ConsoleService::error(err.to_string().as_str()),
                        }
                    }

                    Msg::FetchFailed
                });

            let request = Request::get(&self.user_url).body(Nothing).unwrap();

            let task = FetchService::fetch(request, callback).unwrap();
            self.fetch_task = Some(task);
        }
    }

    fn view(&self) -> Html {
        fn render_failed_to_fetch() -> Html {
            html! {
              <h3>{ "Failed to gather members from GitHub!" }</h3>
            }
        }

        if self.fetch_failed {
            render_failed_to_fetch()
        } else if !self.is_fetching && self.has_member {
            match &self.member {
                Some(member) => {
                    fn render_description(member: &User) -> Html {
                        let bio = match &member.bio {
                            Some(bio) => bio,
                            None => "",
                        };
                        html! {
                            <>
                                <h1>{ member.name.to_string() }</h1>
                                <p class="contributor">
                                    {bio}
                                </p>
                                </>
                        }
                    }
                    html! {{
                        render_description(member)
                    }}
                }
                _ => Html::default(),
            }
        } else {
            html! {
              <h1>{ "Nothing to see here!" }</h1>
            }
        }
    }
}
