use yew::prelude::*;
use yew_router::prelude::*;

mod components;
use components::{Flower, FlowerGarden, ImageDir, Form};

pub fn server_ip() -> AttrValue {
    AttrValue::from(my_server_ip())
}

pub fn my_server_ip() -> &'static str {
    "http://coxs.top:64009"
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/flower")]
    Flower,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Root /> },
        Route::Flower => html! {
            <FlowerGarden>
                <Flower form_title="Set background" api="/api/set_background" />
                <Flower form_title="Upload image" api="/api/save_image" />
            </FlowerGarden>
        },
        Route::NotFound => html! { "404" },
    }
}

#[function_component]
fn Root() -> Html {
    html! {
        <ImageDir api="/api/get_image_dir" />
    }
}

#[function_component]
fn App() -> Html {
    //let flower_vector = vec![html!{<Flower/>}; 10];
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
