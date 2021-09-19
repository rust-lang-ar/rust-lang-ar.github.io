use super::member_description::MemberDescription;
use anyhow::Error;
use serde::Deserialize;
use yew::format::Nothing;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::ConsoleService;

/// URL to fetch members for **rust-lang-ve** organization from GitHub
const GITHUB_MEMBERS_URL: &str = "https://api.github.com/orgs/rust-lang-ar/members";

#[derive(Deserialize, Debug)]
pub struct User {
    pub login: String,
    pub avatar_url: String,
    pub url: String,
}

pub struct OutMembers {
    is_fetching: bool,
    fetch_failed: bool,
    fetch_task: Option<FetchTask>,
    has_members: bool,
    link: ComponentLink<Self>,
    members: Option<Vec<User>>,
}

pub enum Msg {
    FetchFailed,
    FetchSuccess(Vec<User>),
}

impl Component for OutMembers {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            is_fetching: false,
            fetch_failed: false,
            fetch_task: None,
            has_members: false,
            link,
            members: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchSuccess(members) => {
                if !members.is_empty() {
                    self.has_members = true;
                }

                self.is_fetching = false;
                self.members = Some(members);

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
        false
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

                                if let Ok(members) = serde_json::from_str::<Vec<User>>(body) {
                                    return Msg::FetchSuccess(members);
                                }
                            }
                            Err(err) => ConsoleService::error(err.to_string().as_str()),
                        }
                    }

                    Msg::FetchFailed
                });

            let request = Request::get(GITHUB_MEMBERS_URL).body(Nothing).unwrap();

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
        } else if !self.is_fetching && self.has_members {
            match &self.members {
                Some(members) => {
                    fn render_member(login: &str, avatar_url: &str, user_url: &str) -> Html {
                        let alt_text = format!("{} GitHub Profile Picture", login);
                        let user_profile_url = format!("https://github.com/{}", login);

                        html! {
                          <li class="contributor">
                            <a href=user_profile_url.to_string() target="_blank">
                              <img
                                src=avatar_url.to_string()
                                alt=alt_text
                                class="rounded-full"
                                height="260"
                                width="260"
                              />
                            </a>
                            <MemberDescription user_url=user_url.to_string() />
                          </li>
                        }
                    }

                    html! {
                        <section class="our-members-section">
                        <div class="our-members-title-container">
                            <span class="our-members-title">{"Miembros"}</span>
                        </div>
                        <ul class="our-members-list">
                            {
                            for members.iter().map(|member| {
                                render_member(&member.login, &member.avatar_url, &member.url)
                            })
                            }
                        </ul>
                      </section>
                    }
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
