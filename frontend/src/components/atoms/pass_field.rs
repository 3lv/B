use yew::prelude::*;
use web_sys::wasm_bindgen::UnwrapThrowExt;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub onchange: Callback<String>,
}

#[function_component]
pub fn PassInput(props: &Props) -> Html {
    let handle_onchange = props.onchange.clone();
    let onchange = Callback::from({
        move |input_event: Event| {
            let target: HtmlInputElement = input_event
                .target()
                .unwrap_throw()
                .dyn_into()
                .unwrap_throw();
            //web_sys::console::log_1(&target.value().into()); // <- can console the value.
            handle_onchange.emit(target.value());
        }
    });
    html! {
        <input type="password" name={props.name.clone()} {onchange} />
    }
}
