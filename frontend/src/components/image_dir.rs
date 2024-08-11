use yew::prelude::*;
use crate::components::molecules::image_card::ImageCard;

use web_sys::console;

use js_sys::JsString;

use crate::server_ip;

#[derive(Properties, PartialEq)]
pub struct ImageDirProps {
    #[prop_or_else(server_ip)]
    pub ip: AttrValue,
    pub api: AttrValue
}

#[derive(PartialEq)]
enum ImageDirError {
    NeedsFetch(),
    Loading(),
    CantLoad(String),
}

struct ImageDirState {
    images: Result<Vec<String>, ImageDirError>
}

#[function_component]
pub fn ImageDir(ImageDirProps{ip, api}: &ImageDirProps) -> Html {
    let state = use_state(|| ImageDirState {
        images: Err(ImageDirError::NeedsFetch())
    });
    let client = reqwest::Client::new();
    let images_endpoint = format!(
        "{0}{1}", ip, api
    );
    if state.images == Err(ImageDirError::NeedsFetch()) {
    let state_clone = state.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let state = state_clone;
        state.set(ImageDirState {
            images: Err(ImageDirError::Loading()) }
        );
        match client.get(images_endpoint)
            .send()
            .await {
            Ok(res) => {
                match res.json::<Vec<String>>().await {
                    Ok(fetched_images) => {
                        state.set(ImageDirState {
                            images: Ok(fetched_images)
                        });
                        console::log_1(&JsString::from("Images loaded"));
                    }
                    Err(err) => {
                        state.set(ImageDirState {
                            images: Err(ImageDirError::CantLoad(err.to_string()))
                        });
                    }
                }
            }
            Err(err) => {
                state.set(ImageDirState {
                    images: Err(ImageDirError::CantLoad("Api not available".to_string() + &err.to_string()))
                });
            }
        }
    });
    }
    html! {
        <div class="image-dir"> {
    match &state.images {
        Ok(images) => {
                images.iter().map(|image| html! {
                    <ImageCard src={image.clone()} />
                }).collect::<Html>()
        }
        Err(err) => {
            use ImageDirError as Err;
            match err {
                Err::Loading() | Err::NeedsFetch() => {
                    html! { "Images are loading..." }
                }
                Err::CantLoad(string) => {
                    html! { string.clone() }
                }
            }
        }
    }
    } </div>
    }
}
