use yew::prelude::*;
use web_sys::MouseEvent;

use crate::components::ImageForm;
#[derive(Properties, PartialEq)]
pub struct FlowerProps {
    #[prop_or_default]
    pub children: Html,
    pub api: AttrValue,
}
struct FlowerState {
    is_open: bool,
}
#[function_component]
pub fn Flower(props: &FlowerProps) -> Html {
    let state = use_state(|| FlowerState {
        is_open: true,
    });
    let onclick = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            state.set(FlowerState {
                is_open: false,
            });
        })
    };
    let mut classes = vec!["flower"];
    if state.is_open == true {
        classes.push("opened");
    }
    html! {
        <div class="flower-frame">
            <div class={classes!(classes)} id="flower_chbg">
                <div class="center" {onclick}></div>
                <div class="petal petal1"></div>
                <div class="petal petal2"></div>
                <div class="petal petal3"></div>
                <div class="petal petal4"></div>
                <div class="petal petal5"></div>
                <div class="petal petal6"></div>
                //{ children.clone() }
            </div>
            <ImageForm action={props.api.clone()} is_open={!state.is_open} />
        </div>
    }
}
