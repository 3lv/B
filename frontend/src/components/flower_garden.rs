use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn FlowerGarden(Props{children}: &Props) -> Html {
    html! {
        //<div class="FlowerGarden" style="display: grid; grid-gap: 1rem; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));">
        <div class="FlowerGarden" style="display: flex; justify-content: center; align-items: center;">
            {children.clone()}
        </div>
    }
}
