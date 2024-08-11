use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ImageProps {
    pub src: AttrValue,
    #[prop_or(AttrValue::from("Image did not load"))]
    pub alt: AttrValue,
}

#[function_component]
pub fn Image(props: &ImageProps) -> Html {
    html! {
        <a href={props.src.clone()}>
            <img src={props.src.clone()} alt={props.alt.clone()} />
        </a>
    }
}
