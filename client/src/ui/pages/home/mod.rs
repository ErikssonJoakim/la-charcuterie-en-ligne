use crate::ui::components::input::{Input, TypeValue};
use yew::{function_component, html, Callback, Html};

#[function_component(Home)]
pub fn home() -> Html {
    let onclick = Callback::from(|_| {
        let greeting = String::from("Hi there");
        web_sys::console::log_1(&greeting.into());
    });

    html! {
    <main>
        <Input value={TypeValue::Number(3.14)} />
        <button {onclick}>{ "Click" }</button>
    </main>
    }
}
