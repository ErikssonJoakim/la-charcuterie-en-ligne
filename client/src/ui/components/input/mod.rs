use yew::prelude::{function_component, html, Html, Properties};

struct InputAttributes {
    r#type: &'static str,
    value: String,
}

#[derive(Clone, PartialEq)]
pub enum TypeValue {
    Number(f64),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: Option<TypeValue>,
}

#[function_component(Input)]
pub fn input(props: &Props) -> Html {
    let Props { value } = props.clone();

    let input = match value {
        Some(TypeValue::Number(value)) => InputAttributes {
            r#type: "number",
            value: value.to_string(),
        },
        None => InputAttributes {
            r#type: "text",
            value: "".to_string(),
        },
    };

    html! {
    <input type={input.r#type} value={input.value} />
    }
}
