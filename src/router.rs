use yew::{html::IntoPropValue, web_sys::Url};
use yew_router::{components::RouterAnchor, prelude::*, switch::Permissive};

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/events"]
    Events,
    #[to = "/about"]
    About,
    #[to = "/!"]
    Index,
}
impl AppRoute {
    pub fn into_public(self) -> PublicUrlSwitch {
        PublicUrlSwitch(self)
    }

    pub fn into_route(self) -> Route {
        Route::from(self.into_public())
    }
}

#[derive(Clone, Debug)]
pub struct PublicUrlSwitch(AppRoute);
impl PublicUrlSwitch {
    fn base_url() -> Url {
        if let Ok(Some(href)) = yew::utils::document().base_uri() {
            Url::new(&href).unwrap()
        } else {
            Url::new("/").unwrap()
        }
    }

    fn base_path() -> String {
        let mut path = Self::base_url().pathname();
        if path.ends_with('/') {
            path.pop();
        }

        path
    }

    pub fn route(self) -> AppRoute {
        self.0
    }
}
impl Switch for PublicUrlSwitch {
    fn from_route_part<STATE>(part: String, state: Option<STATE>) -> (Option<Self>, Option<STATE>) {
        if let Some(part) = part.strip_prefix(&Self::base_path()) {
            let (route, state) = AppRoute::from_route_part(part.to_owned(), state);
            (route.map(Self), state)
        } else {
            (None, None)
        }
    }

    fn build_route_section<STATE>(self, route: &mut String) -> Option<STATE> {
        route.push_str(&Self::base_path());
        self.0.build_route_section(route)
    }
}


impl IntoPropValue<PublicUrlSwitch> for AppRoute {
    fn into_prop_value(self: AppRoute) -> PublicUrlSwitch {
        self.into_public()
    }
}

pub type AppRouter = Router<PublicUrlSwitch>;
pub type AppAnchor = RouterAnchor<PublicUrlSwitch>;