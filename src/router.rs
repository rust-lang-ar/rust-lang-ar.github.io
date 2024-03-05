use yew_router::prelude::*;

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/resources"]
    Resources,
    #[to = "/events"]
    Events,
    #[to = "/about"]
    About,
    #[to = "/"]
    Index,
}
