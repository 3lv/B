use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    //#[prop_or_default]
    pub action: AttrValue,
    #[prop_or(false)]
    pub is_open: bool,
}

#[function_component]
pub fn ImageForm(Props{ref action, is_open}: &Props) -> Html {
    let mut classes = vec!["form-container"];
    if *is_open == true {
        classes.push("opened");
    }
    html! {
        <div class={classes!(classes)}>
        <form method="post" enctype="multipart/form-data" action={action}>
            <input type="file" name="image" />
            <input type="submit" value="Submit" />
        </form>
        </div>
    }
}
