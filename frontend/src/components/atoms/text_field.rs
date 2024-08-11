use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
}

#[function_component]
pub fn TextInput(props: &Props) -> Html {
    html! {
        <input type="text" name={props.name.clone()} />
    }
}
