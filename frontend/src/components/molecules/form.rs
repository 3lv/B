use yew::prelude::*;
use std::ops::Deref;

use web_sys::console;
use js_sys::JsString;

use crate::components::atoms::pass_field::PassInput;

#[derive(Default, Clone)]
struct FormData {
    pub password: String,
}

#[function_component]
pub fn Form() -> Html {
    let state = use_state(|| FormData::default());
    let state_clone = state.clone();
    let password_changed = Callback::from(move |new_password: String| {
        let mut data = state_clone.deref().clone();
        data.password = new_password;
        state_clone.set(data);
        console::log_1(&JsString::from("password updated"));
    });
    html! {
        <div class="custom_form">
            <PassInput name="password" onchange={password_changed} />
            <p>{format!("Password: {}", state.password)}</p>
        </div>
    }
}
