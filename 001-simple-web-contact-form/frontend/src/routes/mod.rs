use yew_router::prelude::*;
use yew_router::switch::Permissive;

pub mod home;

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
	#[to = "/!"]
	Home,
	#[to = "/notfound!"]
	PageNotFound(Permissive<String>),
}
