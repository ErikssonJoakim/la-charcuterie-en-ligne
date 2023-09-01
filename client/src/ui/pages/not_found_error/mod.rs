use yew::{function_component, html, Html};
use yew_router::prelude::use_location;

#[function_component(NotFoundError)]
pub fn not_found_error() -> Html {
    let location = use_location().unwrap();
    let path = location.path();

    html! {
        <div>
            <h1>{format!("The path {} does not exist", path) }</h1>
        </div>
    }
}
