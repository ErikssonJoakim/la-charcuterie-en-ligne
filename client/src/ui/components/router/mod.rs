use yew::{prelude::html, Html};
use yew_router::prelude::Routable;

use crate::ui::pages::{home::Home, not_found_error::NotFoundError};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/*path")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::NotFound => html! { <NotFoundError /> },
    }
}
