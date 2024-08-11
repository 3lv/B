use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    //#[prop_or_default]
    pub title: AttrValue,
    pub action: AttrValue,
    #[prop_or(false)]
    pub is_open: bool,
}

#[function_component]
pub fn ImageForm(props: &Props) -> Html {
    let mut classes = vec!["form-container"];
    if props.is_open == true {
        classes.push("opened");
    }
    html! {
        <div class={classes!(classes)}>
        <h2> {&props.title} </h2>
        <form method="post" enctype="multipart/form-data" action={&props.action}>
            <input type="file" name="image" />
            <input type="password" name="password" placeholder="password" />
            <input type="submit" value="Submit" />
        </form>
        </div>
    }
}
