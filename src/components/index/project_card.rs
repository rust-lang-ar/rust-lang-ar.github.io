use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lang {
    Rust,
    Css,
    JavaScript,
    Html,
    NoLanguage,
}

impl Lang {
    /// Retrieve the class and the text for the repository
    /// language field as a tuple where `0` is the class name
    /// and `1` the label name
    pub fn markup_resources(&self) -> (String, String) {
        match self {
            Lang::Rust => (String::from("rust"), String::from("Rust")),
            Lang::Css => (String::from("css"), String::from("CSS")),
            Lang::JavaScript => (String::from("javascript"), String::from("JavaScript")),
            Lang::Html => (String::from("html"), String::from("HTML")),
            Lang::NoLanguage => (String::default(), String::from("No language")),
        }
    }
}

impl From<String> for Lang {
    fn from(string: String) -> Self {
        match string.to_lowercase().as_str() {
            "rust" => Lang::Rust,
            "html" => Lang::Html,
            "javascript" => Lang::JavaScript,
            "css" => Lang::Css,
            _ => Lang::NoLanguage,
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub text: String,
    pub title: String,
    pub language: Option<String>,
    pub stargazers_count: u32,
    pub html_url: String,
}

pub struct ProjectCard {
    pub description: String,
    pub language: Lang,
    pub name: String,
    pub stargazers_count: u32,
    pub html_url: String,
}

impl Component for ProjectCard {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        let mut lang: Lang = Lang::NoLanguage;

        if let Some(language) = props.language {
            lang = Lang::from(language);
        }

        Self {
            description: props.text,
            language: lang,
            name: props.title,
            stargazers_count: props.stargazers_count,
            html_url: props.html_url,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn view(&self) -> Html {
        let language_text = self.language.markup_resources().1;

        html! {
            <a href={self.html_url.clone()} class="h-full">
                <button class="project-card-container">
                    <h2 class="project-card-header">{ self.name.clone() }</h2>
                    <p class="project-card-description">{ self.description.clone() }</p>
                    <div class="project-card-footer">
                        <div class="h-5">
                            <div class="project-card-lang-circle"></div>
                        </div>
                        <p>{language_text}</p>
                    </div>
                </button>
            </a>
        }
    }
}
