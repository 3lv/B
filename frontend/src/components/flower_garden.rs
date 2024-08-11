use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn FlowerGarden(Props{children}: &Props) -> Html {
    let style="display: flex; flex-direction: column; justify-content: space-around; align-items: center;";
    html! {
        //<div class="FlowerGarden" style="display: grid; grid-gap: 1rem; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));">
        <div class="FlowerGarden" {style} >
            {children.clone()}
        </div>
    }
}
