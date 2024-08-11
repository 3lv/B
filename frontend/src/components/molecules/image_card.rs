use yew::prelude::*;
use crate::components::atoms::Image;

#[derive(Properties, PartialEq)]
pub struct ImageCardProps {
    pub src: AttrValue,
    #[prop_or(AttrValue::from("Image did not load"))]
    pub alt: AttrValue,
}

#[function_component]
pub fn ImageCard(props: &ImageCardProps) -> Html {
    html! {
        <a class="image-card" href={props.src.clone()}>
            <Image src={props.src.clone()} />
            <p> {"Imaginea nu are descriere"} </p>
        </a>
    }
}
